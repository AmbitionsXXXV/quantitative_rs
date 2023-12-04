use chrono::NaiveDate;
use csv::Reader;
use encoding_rs::GBK;
use serde::{de, Deserialize};
use std::fs::read;
use std::io::Cursor;

// 因为相对路径是根据执行目录来计算的，所以这里的 csv 文件路径是相对于执行目录的，放在子项目根目录读取不到，除非在该子目录下执行 cargo run
// const CSV_PATH: &str = "./588800.SH.csv";
// 要使用相对路径读取子项目的 csv 文件，则需要使用如下相对路径
const CSV_PATH: &str = "./588460.SH.csv";

#[derive(Default, Debug, Clone, Deserialize)]
struct DataFrame {
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

fn main() {
    // 以二进制格式读取整个文件内容
    let data = read(CSV_PATH).expect("无法读取文件");

    // 将GBK编码的数据转换为UTF-8
    let (cow, _, _) = GBK.decode(&data);

    // 创建一个内存中的Cursor，以便csv库可以从中读取
    let cursor = Cursor::new(cow.as_bytes());

    // 使用Cursor创建CSV读取器
    let mut rdr = Reader::from_reader(cursor);

    // 读取 csv 数据，并反序列化为 struct
    rdr.deserialize().for_each(|r| {
        let record: DataFrame = r.unwrap();
        println!("record: {:?}", record);
        // println!("stock_name: {:?}", record.stock_name.unwrap());
        // println!("trade_date: {:?}", record.trade_date);
    });
}
