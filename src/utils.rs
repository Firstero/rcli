use anyhow::Result;
use std::io::Read;

// 提取 get_reader 函数, 用于根据输入的文件路径或者 - 来获取 Reader
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    match input {
        "-" => Ok(Box::new(std::io::stdin())),
        path => Ok(Box::new(std::fs::File::open(path)?)),
    }
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    Ok(content)
}
