// 在 main.rs
use polars::prelude::*;
mod convert;

// const CSV_PATH: &str = "./588460.SH.csv";
const NEW_CSV_PATH: &str = "./2023-12-03_new.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 CSV 文件
    let df = CsvReader::from_path(NEW_CSV_PATH)?.finish()?;

    // 替换列名
    // df.set_column_names(TABLE_HEADERS)?;
    let g = df.group_by(["股票代码"])?.select(["发布日期"]).count()?;

    // 使用 select 根据列名选择列，再使用 head 选择前 10 行
    // df = df.select(["交易日期"])?.head(Some(10));

    println!("{:?}", g);

    Ok(())
}
