use chrono::NaiveDate;
use csv::Reader;
use encoding_rs::GBK;
use serde::{de, Deserialize};
use std::fs::read;
use std::io::Cursor;

#[derive(Default, Debug, Clone, Deserialize)]
struct DataFrameItem {
    // 对应 csv 文件的列名
    #[serde(deserialize_with = "date_from_str")]
    trade_date: NaiveDate,
    volume: Option<f64>,
    turnover: Option<f64>,
    fund_code: Option<String>,
    lowest_price: Option<f64>,
    nav_per_unit: Option<f64>,
    highest_price: Option<f64>,
    closing_price: Option<f64>,
    opening_price: Option<f64>,
    stock_name: Option<String>,
    turnover_rate: Option<f64>,
    post_adjustment_factor: Option<f64>,
    previous_closing_price: Option<f64>,
    accumulated_nav_per_unit: Option<f64>,
}

fn date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

#[derive(Debug)]
pub struct DataFrame {
    data: Vec<DataFrameItem>,
}

impl From<&str> for DataFrame {
    fn from(path: &str) -> Self {
        let mut _data: Vec<DataFrameItem> = Vec::new();
        // 以二进制格式读取整个文件内容
        let data = read(path).expect("无法读取文件");

        // 将GBK编码的数据转换为UTF-8
        let (cow, _, _) = GBK.decode(&data);

        // 创建一个内存中的Cursor，以便csv库可以从中读取
        let cursor = Cursor::new(cow.as_bytes());

        // 使用Cursor创建CSV读取器
        let mut csv_data = Reader::from_reader(cursor);

        csv_data.deserialize().for_each(|r| {
            if let Ok(row) = r {
                _data.push(row)
            }
            // println!("stock_name: {:?}", record.stock_name.unwrap());
            // println!("trade_date: {:?}", record.trade_date);
        });

        DataFrame { data: _data }
    }
}
