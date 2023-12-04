use chrono::NaiveDate;
use comfy_table::Table;
use csv::Reader;
use encoding_rs::GBK;
use serde::{de, Deserialize};
use std::fs::read;
use std::io::Cursor;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DataFrameItem {
    // 对应 csv 文件的列名
    #[serde(deserialize_with = "date_from_str")]
    trade_date: NaiveDate, // 交易日期
    volume: Option<f64>,                     // 成交量
    turnover: Option<f64>,                   // 成交额
    fund_code: Option<String>,               // 基金代码
    lowest_price: Option<f64>,               // 最低价
    nav_per_unit: Option<f64>,               // 单位净值
    highest_price: Option<f64>,              // 最高价
    closing_price: Option<f64>,              // 收盘价
    opening_price: Option<f64>,              // 开盘价
    stock_name: Option<String>,              // 股票名字
    turnover_rate: Option<f64>,              // 换手率
    post_adjustment_factor: Option<f64>,     // 前复权因子
    pub previous_closing_price: Option<f64>, // 前收盘价
    accumulated_nav_per_unit: Option<f64>,   // 累计净值
    #[serde(skip)]
    ma5: f64,              // 5日均线
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

const TABLE_HEADERS: &[&str] = &[
    "交易日期",
    "基金名字",
    "股票代码",
    "前收盘价",
    "开盘价",
    "收盘价",
    "最高价",
    "最低价",
    "成交量",
    "成交额",
    "换手率",
    "前复权因子",
    "累计净值",
    "单位净值",
    "振幅",
];

pub enum SortField {
    Volume,
    Turnover,
    TradeDate,
    NavPerUnit,
    LowestPrice,
    HighestPrice,
    OpeningPrice,
    ClosingPrice,
    TurnoverRate,
    PostAdjustmentFactor,
    PreviousClosingPrice,
    AccumulatedNavPerUnit,
}

type FilterFn = Box<dyn Fn(&DataFrameItem) -> bool>;

impl DataFrame {
    // 根据日期排序
    pub fn sort(&mut self, field: SortField, ascending: bool) -> &mut Self {
        let comparator = |a: &DataFrameItem, b: &DataFrameItem| match field {
            SortField::TradeDate => a.trade_date.cmp(&b.trade_date),
            SortField::Volume => a
                .volume
                .partial_cmp(&b.volume)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortField::Turnover => a
                .turnover
                .partial_cmp(&b.turnover)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortField::NavPerUnit => a
                .nav_per_unit
                .partial_cmp(&b.nav_per_unit)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortField::LowestPrice => a
                .lowest_price
                .partial_cmp(&b.lowest_price)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortField::HighestPrice => todo!(),
            SortField::OpeningPrice => todo!(),
            SortField::ClosingPrice => todo!(),
            SortField::TurnoverRate => todo!(),
            SortField::PostAdjustmentFactor => todo!(),
            SortField::PreviousClosingPrice => todo!(),
            SortField::AccumulatedNavPerUnit => todo!(),
        };

        if ascending {
            self.data.sort_by(comparator);
        } else {
            self.data.sort_by(|a, b| comparator(b, a));
        }

        self
    }

    pub fn print(&mut self, include_ma5: bool, filter: Option<FilterFn>) {
        let mut table = Table::new();

        // 根据 include_ma5 决定是否添加 ma5 列
        let mut headers = Vec::from(TABLE_HEADERS);
        if include_ma5 {
            headers.push("5日均线");

            self.calc_ma5();

            // 时间倒序排
            self.sort(SortField::TradeDate, false);
        }

        let mut data: Vec<DataFrameItem> = self.data.clone();
        if let Some(f) = filter {
            data = data.into_iter().filter(f).collect();
        }

        table.set_header(headers);

        data.iter().for_each(|row| {
            let amp = (row.highest_price.unwrap_or_default()
                - row.lowest_price.unwrap_or_default())
                / row.previous_closing_price.unwrap_or_default()
                * 100.0;
            let amp = format!("{:.2}%", amp);

            // 根据 include_ma5 决定是否添加 ma5 值
            let mut row_data = vec![
                row.trade_date.to_string(),
                row.stock_name.clone().unwrap_or_default(),
                row.fund_code.clone().unwrap_or_default(),
                row.previous_closing_price.unwrap_or_default().to_string(),
                row.opening_price.unwrap_or_default().to_string(),
                row.closing_price.unwrap_or_default().to_string(),
                row.highest_price.unwrap_or_default().to_string(),
                row.lowest_price.unwrap_or_default().to_string(),
                row.volume.unwrap_or_default().to_string(),
                row.turnover.unwrap_or_default().to_string(),
                row.turnover_rate.unwrap_or_default().to_string(),
                row.post_adjustment_factor.unwrap_or_default().to_string(),
                row.accumulated_nav_per_unit.unwrap_or_default().to_string(),
                row.nav_per_unit.unwrap_or_default().to_string(),
                amp,
            ];

            if include_ma5 {
                let ma5 = format!("{:.2}", row.ma5);
                row_data.push(ma5);
            }

            table.add_row(row_data);
        });

        println!("{table}");
    }

    fn calc_ma5(&mut self) {
        let len = self.data.len();

        for index in 0..len {
            if index + 5 > len {
                break;
            }

            let mut sum = 0.0;

            for i in 0..5 {
                sum += self
                    .data
                    .get(index + i)
                    .unwrap()
                    .closing_price
                    .unwrap_or_default();
            }

            let sum = sum / 5.0; // 计算均值
            self.data.get_mut(index).unwrap().ma5 = sum // 将均值赋值给 ma5 字段
        }
    }
}
