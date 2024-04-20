use std::fs;

use csv::Reader;
use serde_json::Value;

use crate::opts::OutputFormat;

//   Name          │      Position      │        DOB        │    Nationality     │ Kit Number │
//  varchar        │      varchar       │      varchar      │      varchar       │  int64     |
// 按照上述结构定义一个 Record 结构体
pub fn process_csv(input: &str, output: String, output_format: OutputFormat) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(100);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value)
    }
    let contents = match output_format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, contents)?;
    Ok(())
}
