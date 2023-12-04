use encoding_rs::GB18030;
use polars::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use tempfile::tempfile;

pub trait GbkToUtf8Converter {
    fn convert(&self, src_path: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Converter;

impl GbkToUtf8Converter for Converter {
    fn convert(&self, src_path: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 读取GBK编码的CSV文件
        let mut file = File::open(src_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // 将GBK编码转换为UTF-8
        let (cow, _, _) = GB18030.decode(&buffer);
        let data_utf8 = cow.as_ref();

        // 将UTF-8数据写入临时文件
        let mut temp_file = tempfile()?;
        temp_file.write_all(data_utf8.as_bytes())?;

        // 使用Polars从临时文件读取数据
        let reader = BufReader::new(temp_file);
        let mut df = CsvReader::new(reader).finish()?;

        // 将DataFrame以UTF-8格式写入新的CSV文件
        let mut file = File::create(dest_path)?;
        CsvWriter::new(&mut file).finish(&mut df)?;

        Ok(())
    }
}
