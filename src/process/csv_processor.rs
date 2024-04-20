use std::fs;

use csv::Reader;

use crate::opts::OutputFormat;

// 将 csv 文件转换为 json 或者 yaml 格式
pub fn process(input: &str, output: String, output_format: OutputFormat) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(100);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value)
    }
    let contents = match output_format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, contents)?;
    Ok(())
}
