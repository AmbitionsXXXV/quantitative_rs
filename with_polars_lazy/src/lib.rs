use polars::prelude::*;

/// 过滤股票代码为 sh6002 开头的数据
pub fn filter_csv_lazy(df: DataFrame, patron: &str) -> Result<DataFrame, PolarsError> {
    let print_exp = [col("发布日期").alias("pub_date")];

    let g = df
        .lazy()
        .filter(col("股票代码").str().contains(lit(patron), false))
        .collect()?;

    // 增加一个 select 操作，只选择发布日期列
    let ret = g.lazy().select(print_exp).collect()?;

    Ok(ret)
}
