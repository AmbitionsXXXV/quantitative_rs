// 在 main.rs
use polars::prelude::*;
mod calc;
mod convert;

use crate::calc::{calc_amp, calc_ma};

// const CSV_PATH: &str = "./588460.SH.csv";
const NEW_CSV_PATH: &str = "./2023-12-03_new.csv";

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

    calc_amp(&mut df)?;

    calc_ma(&mut df, 5, "MA5")?;
    calc_ma(&mut df, 20, "MA20")?;

    // 使用 select 根据列名选择列，再使用 head 选择前 10 行
    // df = df.select(["交易日期"])?.head(Some(10));

    println!("{:?}", df);

    Ok(())
}
