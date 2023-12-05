use polars::prelude::*;

/// 过滤股票代码为 sh6002 开头的数据
pub fn filter_csv_lazy(
    df: DataFrame,
    patron: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let g = df
        .lazy()
        .filter(col("股票代码").str().contains(lit(patron), false))
        .collect()?;

    Ok(g)
}
