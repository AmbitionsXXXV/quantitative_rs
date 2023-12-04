mod dataframe;
use dataframe::dataframe::{DataFrame, DataFrameItem, SortField};

// 因为相对路径是根据执行目录来计算的，所以这里的 csv 文件路径是相对于执行目录的，放在子项目根目录读取不到，除非在该子目录下执行 cargo run
const CSV_PATH: &str = "./588460.SH.csv";
// 要使用相对路径读取子项目的 csv 文件，则需要使用如下相对路径
// const CSV_PATH: &str = "./csv_read/588460.SH.csv";

fn main() {
    let mut df = DataFrame::from(CSV_PATH);
    // println!("{:?}", df);
    let filter = |df: &DataFrameItem| df.previous_closing_price.unwrap_or(0.0) > 0.9;

    df.sort(SortField::TradeDate, true)
        .print(true, Some(Box::new(filter)));
}
