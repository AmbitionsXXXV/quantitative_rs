use polars::prelude::*;
use with_polars_lazy::filter_csv_lazy;

const NEW_CSV_PATH: &str = "./2023-12-03_new.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let df = CsvReader::from_path(NEW_CSV_PATH)?.finish()?;

    // 使用 LazyCsvReader 读取 CSV 文件
    let df = LazyCsvReader::new(NEW_CSV_PATH)
        .has_header(true)
        .finish()?
        .collect()?;

    let df = filter_csv_lazy(df, "^sh6002")?;

    println!("{:?}", df);
    // eager api 与 lazy api 的区别
    // eager api 会立即执行，而 lazy api 会在调用 collect 时才执行
    // eager api 使用 group_by
    // let g = df.group_by(["股票代码"])?.count()?;

    // lazy api 使用 group_by
    // let lg = df
    //     .lazy()
    //     .group_by(["股票代码"])
    //     .agg([col("发布日期").alias("数量").count()])
    //     .collect()?;

    Ok(())
}
