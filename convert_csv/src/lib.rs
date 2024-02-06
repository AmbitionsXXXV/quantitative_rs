use encoding_rs::GBK;
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
        // 读取 GBK 编码的 CSV 文件
        let mut file = File::open(src_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // 将 GBK 编码转换为 UTF-8
        let (cow, _, _) = GBK.decode(&buffer);
        let data_utf8 = cow.as_ref();

        // 将 UTF-8 数据写入临时文件
        let mut temp_file = tempfile()?;
        temp_file.write_all(data_utf8.as_bytes())?;

        // 使用 Polars 从临时文件读取数据
        let reader = BufReader::new(temp_file);
        let mut df = CsvReader::new(reader).finish()?;

        // 将 DataFrame 以 UTF-8 格式写入新的 CSV 文件
        let mut file = File::create(dest_path)?;
        CsvWriter::new(&mut file).finish(&mut df)?;

        Ok(())
    }
}
