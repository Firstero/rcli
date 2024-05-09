use std::{ffi::OsStr, net::SocketAddr, path::PathBuf, time::SystemTime};

use anyhow::Result;
use askama::Template;
use axum::{
    body::Body,
    extract,
    http::{header, HeaderValue, Request, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use base64::Engine;
use tokio::{fs, io};
use tower::util::ServiceExt;
use tower_http::{normalize_path::NormalizePath, services::ServeDir};

pub async fn process_http_serve(root_dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(
        "Rcli Static File Serving directory {:?} on addr {}",
        root_dir,
        addr
    );
    let serve_base_dir = root_dir.to_string_lossy().to_string();
    // let state = HttpServeState { root_dir };
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = Router::new()
        .nest_service("/", get(move |req| custom_serve_dir(req, serve_base_dir)))
        .route("/favicon.ico", get(favicon))
        .route("/health", get(health_check));

    let app = NormalizePath::trim_trailing_slash(app);
    axum::serve(
        listener,
        axum::ServiceExt::<extract::Request>::into_make_service(app),
    )
    .await?;
    Ok(())
}

/// health check for K8s liveness and readiness probe
async fn health_check() -> impl IntoResponse {
    "ok"
}

async fn favicon() -> impl IntoResponse {
    // one pixel favicon generated from https://png-pixel.com/
    let one_pixel_favicon = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    let pixel_favicon = base64::prelude::BASE64_STANDARD
        .decode(one_pixel_favicon)
        .unwrap();
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
        pixel_favicon,
    )
}

/// Serve a directory, Because the ServeDir service does not support listing directories, we need to implement it ourselves.
/// If the path is a directory, list its contents.
/// If the path is a file, use ServiDir to serve it.
/// Because ServeDir in nested route causes invalid redirects #1731 https://github.com/tokio-rs/axum/issues/1731, handle StatusCode::TEMPORARY_REDIRECT manually here to avoid the problem.
async fn custom_serve_dir(
    req: Request<Body>,
    serve_base_dir: String,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut path = req.uri().path().trim_start_matches('/').to_owned();
    if path.is_empty() {
        path = ".".to_string();
    }
    let full_path = PathBuf::from(&serve_base_dir).join(&path);
    tracing::debug!(
        "custom_serve_dir get req_path is {:?} and serve path={:?}",
        path,
        full_path
    );

    let service = ServeDir::new(&serve_base_dir);
    let result = service.oneshot(req).await;
    match result {
        Ok(res) => match res.status() {
            StatusCode::NOT_FOUND | StatusCode::TEMPORARY_REDIRECT
                if PathBuf::from(&full_path).is_dir() =>
            {
                let rs = visit_dir_one_level(&full_path, &serve_base_dir).await;
                match rs {
                    Ok(files) => Ok(DirListTemplate {
                        lister: DirLister { files },
                        cur_path: path.to_string(),
                    }
                    .into_response()),
                    Err(e) => Ok(ErrorTemplate {
                        err: ResponseError::InternalError(e.to_string()),
                        cur_path: path.to_string(),
                        message: e.to_string(),
                    }
                    .into_response()),
                }
            }
            StatusCode::NOT_FOUND => Ok(ErrorTemplate {
                err: ResponseError::FileNotFound("File Not Found".to_string()),
                cur_path: path.to_string(),
                message: "File Not Found".to_string(),
            }
            .into_response()),
            StatusCode::BAD_REQUEST => Ok(ErrorTemplate {
                err: ResponseError::BadRequest("Bad Request".to_string()),
                cur_path: path.to_string(),
                message: "Bad Request".to_string(),
            }
            .into_response()),
            StatusCode::INTERNAL_SERVER_ERROR => Ok(ErrorTemplate {
                err: ResponseError::BadRequest("Internal Server Error".to_string()),
                cur_path: path.to_string(),
                message: "Internal Server Error".to_string(),
            }
            .into_response()),
            _ => Ok(res.into_response()),
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

async fn visit_dir_one_level(dir_path: &PathBuf, prefix: &str) -> io::Result<Vec<FileInfo>> {
    let mut dir = fs::read_dir(dir_path).await?;
    let mut files: Vec<FileInfo> = Vec::new();

    while let Some(child) = dir.next_entry().await? {
        let the_path = child.path().to_string_lossy().to_string();
        let the_uri_path: String;
        if !prefix.is_empty() && !the_path.starts_with(prefix) {
            tracing::error!("visit_dir_one_level skip invalid path={}", the_path);
            continue;
        } else if prefix != "/" {
            the_uri_path = the_path.strip_prefix(prefix).unwrap().to_string();
        } else {
            the_uri_path = the_path;
        }
        files.push(FileInfo {
            name: child.file_name().to_string_lossy().to_string(),
            ext: PathBuf::from(child.file_name())
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or_default()
                .to_string(),
            mime_type: mime_guess::from_path(child.path())
                .first_or_octet_stream()
                .type_()
                .to_string(),
            path_uri: the_uri_path,
            is_file: child.file_type().await?.is_file(),
            last_modified: child
                .metadata()
                .await?
                .modified()?
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        });
    }

    Ok(files)
}

/// Custom filters for askama template
mod filters {
    pub(crate) fn datetime(ts: &i64) -> ::askama::Result<String> {
        if let Ok(format) =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC")
        {
            return Ok(time::OffsetDateTime::from_unix_timestamp(*ts)
                .unwrap()
                .format(&format)
                .unwrap());
        }
        Err(askama::Error::Fmt(std::fmt::Error))
    }
}

const FAIL_REASON_HEADER_NAME: &str = "Rcli-Http-Serve-Fail-Reason";

pub(crate) enum ResponseError {
    BadRequest(String),
    FileNotFound(String),
    InternalError(String),
}

#[derive(Template)]
#[template(path = "index.html")]
struct DirListTemplate {
    lister: DirLister,
    cur_path: String,
}

struct FileInfo {
    name: String,
    ext: String,
    mime_type: String,
    path_uri: String,
    is_file: bool,
    last_modified: i64,
}

struct DirLister {
    files: Vec<FileInfo>,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    err: ResponseError,
    cur_path: String,
    message: String,
}

impl IntoResponse for ErrorTemplate {
    fn into_response(self) -> Response<Body> {
        let t = self;
        match t.render() {
            Ok(html) => {
                let mut resp = Html(html).into_response();
                match t.err {
                    ResponseError::FileNotFound(reason) => {
                        *resp.status_mut() = StatusCode::NOT_FOUND;
                        resp.headers_mut()
                            .insert(FAIL_REASON_HEADER_NAME, reason.parse().unwrap());
                    }
                    ResponseError::BadRequest(reason) => {
                        *resp.status_mut() = StatusCode::BAD_REQUEST;
                        resp.headers_mut()
                            .insert(FAIL_REASON_HEADER_NAME, reason.parse().unwrap());
                    }
                    ResponseError::InternalError(reason) => {
                        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                        resp.headers_mut()
                            .insert(FAIL_REASON_HEADER_NAME, reason.parse().unwrap());
                    }
                }
                resp
            }
            Err(err) => {
                tracing::error!("template render failed, err={}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {}", err),
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for DirListTemplate {
    fn into_response(self) -> Response<Body> {
        let t = self;
        match t.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("template render failed, err={}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {}", err),
                )
                    .into_response()
            }
        }
    }
}

// 添加单元测试，测试visit_dir_one_level函数
#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_visit_dir_one_level() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path();
        // create a file
        fs::File::create(dir_path.join("a.txt")).unwrap();
        // create a sub directory
        fs::create_dir(dir_path.join("b_dir")).unwrap();
        let mut files = visit_dir_one_level(&dir_path.to_path_buf(), "")
            .await
            .unwrap();
        files.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].name, "a.txt");
        assert_eq!(files[0].ext, "txt");
        assert_eq!(files[0].mime_type, "text");
        assert!(files[0].is_file);
        assert!(files[0].path_uri.ends_with("a.txt"));

        assert_eq!(files[1].name, "b_dir");
        assert_eq!(files[1].ext, "");
        assert_eq!(files[1].mime_type, "application");
        assert!(!files[1].is_file);
        assert!(files[1].path_uri.ends_with("b_dir"));
    }
}
