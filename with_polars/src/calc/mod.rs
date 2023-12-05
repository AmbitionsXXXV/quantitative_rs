use polars::prelude::*;

/// 用于计算振幅
pub fn calc_amp(df: &mut DataFrame) -> Result<(), PolarsError> {
    // 获取列数
    let len = df.width();

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

    // 在尾部添加新列
    df.insert_column(len, amplitude)?;

    Ok(())
}

/// 用于计算移动平均线
pub fn calc_ma(df: &mut DataFrame, days: usize, col: &str) -> Result<(), PolarsError> {
    // 获取列数
    let len = df.width();

    let closing_price = df.column("收盘价")?;

    let opt = RollingOptionsImpl {
        min_periods: days,
        window_size: Duration::new(days as i64),
        ..Default::default()
    };
    let ma5 = closing_price.reverse().rolling_mean(opt)?.reverse();

    let ma5 = Series::new(col, ma5);

    // 在尾部添加新列
    df.insert_column(len, ma5)?;

    Ok(())
}
