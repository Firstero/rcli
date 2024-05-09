# Geektime Rust 训练营

## rcli: 基于Rust实现的命令行工具

### 作业一

#### 作业要求

阅读 chacha20poly1305 文档，了解其使用方法并构建 CLI 对输入文本进行加密 / 解密 CLI接口示例：

- rcli text encrypt -key"xxx"> 加密并输出 base64
- rcli text decrypt -key"XXX" >base64 > binary> 解密文本

#### 实现说明
具体实现，参考 cli/text.rs，运行命令如下
1. 使用chacha20poly1305 crate 进行包装
2. 增加 nonce 参数，用于每次的使用, 参考 chacha20poly1305
- ```rcli text encrypt --key 'keyfile/or stdin' --input textfile --nonce noncefile```
- ```rcli text decrypt --key 'keyfile/or stdin' --input textfile --nonce noncefile```

### 作业二

#### 作业要求

json web token(jwt) 在用户验证领域经常被用到。请构建一个 CLI 来为给定 sub/aud/exp/… 生成一个 jwt。要求生成的 jwt 可以通过 jwt.io 的验证。
CLI接口示例:
- rcli jwt sign --sub acme --aud device1 --exp 14d
- rcli jwt verify -t

#### 实现说明

1. 使用 jsonwebtoken crate 来实现 sign/decode
2. 直接使用 serde_json 来作为 Claims 结构, 参考 jsonwebtoken Validation 结构体, 支持所有标准规范可选字段的定制

#### 使用指南
- rcli jwt sign
```
Usage: rcli jwt sign [OPTIONS]
Options:
      --alg <ALG>  claims algorithm [default: HS384]
      --key <KEY>  sign key file path, or '-' for stdin [default: -]
      --sub <SUB>  subject
      --aud <AUD>  audience
      --iss <ISS>  jwt issuer
      --exp <EXP>  jwt expiration time [default: 1d]
      --nbf <NBF>  jwt nbf time [default: 1d]
      --iat        generate jwt iat or not
  -h, --help       Print help
```
- rcli jwt verify
```
Usage: rcli jwt verify [OPTIONS] --token <TOKEN>
Options:
  -t, --token <TOKEN>                      jwt token
      --key <KEY>                          key file path, or '-' for stdin [default: -]
      --required-claims <REQUIRED_CLAIMS>  required claims
      --aud <AUD>                          audiences
      --iss <ISS>                          issuers
      --sub <SUB>                          jwt subject
      --validate-exp <VALIDATE_EXP>        validate expiration time [possible values: true, false]
      --validate-nbf <VALIDATE_NBF>        validate nbf time [possible values: true, false]
      --validate-aud <VALIDATE_AUD>        validate audience [possible values: true, false]
      --silent                             show token claims, if verified successfully
      --show-self                          show self options
  -h, --help                               Print help
```

### 作业三
#### 作业要求

给课程里的 HTTP 文件服务器添加对 directory index 的支持。

#### 实现说明

1. Service 实现: 使用 CustomServiceDir 包装原始 ServeDir
    - 使用 axum handler 来实现自定义的 CustomServiceDir service
    - handler 实现中基于 tower::ServiceExt 的 oneshot 来包装,实现自定义代理
    - 由于 [ServeDir in nested route causes invalid redirects #1731]( https://github.com/tokio-rs/axum/issues/1731 ), 手动处理了 StatusCode::TEMPORARY_REDIRECT
2. 目录索引生成实现: 使用 tokio::fs 实现目录的一级遍历
3. 前端渲染: 使用 askama 模板引擎来实现目录页面及错误页面渲染，参考 https://github.com/ttys3/static-server

#### 使用指南
- rcli http
```Usage: rcli http <COMMAND>

Commands:
  serve  Serve a directory via HTTP
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
