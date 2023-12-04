// 在 main.rs
use polars::prelude::*;
mod convert;

use polars::{
    io::{csv::CsvReader, SerReader},
    series::Series,
};

// const CSV_PATH: &str = "./588460.SH.csv";
const NEW_CSV_PATH: &str = "./588460_utf8.csv";

const TABLE_HEADERS: &[&str] = &[
    "交易日期",
    "基金名字",
    "股票代码",
    "前收盘价",
    "开盘价",
    "最高价",
    "最低价",
    "收盘价",
    "成交量",
    "成交额",
    "累计净值",
    "单位净值",
    "前复权因子",
    "换手率",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取 CSV 文件
    let mut df = CsvReader::from_path(NEW_CSV_PATH)?.finish()?;

    // 替换列名
    df.set_column_names(TABLE_HEADERS)?;

    // 计算振幅
    let highest_price = df.column("最高价")?;
    let lowest_price = df.column("最低价")?;
    let previous_closing_price = df.column("前收盘价")?;

    let amplitude = highest_price
        .subtract(lowest_price)?
        .divide(previous_closing_price)?
        * 100.0;
    let amplitude: Series = amplitude
        .f64()?
        .into_iter()
        .map(|item| format!("{:.2}%", item.unwrap_or(0.0)))
        .collect();

    let amplitude = Series::new("振幅", amplitude);

    //
    let len = df.width();

    // 在尾部添加新列
    df.insert_column(len, amplitude)?;

    // 使用 select 根据列名选择列，再使用 head 选择前 10 行
    // df = df.select(["交易日期"])?.head(Some(10));

    println!("{:?}", df);

    Ok(())
}
