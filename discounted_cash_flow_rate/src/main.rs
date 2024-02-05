use std::io;
use std::io::Write; // 导入 Write trait，以便使用 flush 方法

/// 前置知识
/// 现金折现率（Discounted Cash Flow Rate，简称DCF）是一种评估投资项目价值的方法，主要用于投资分析、资本预算和企业价值评估中。这种方法的核心思想是将未来现金流（包括收入和支出）按照一定的折现率（即现金折现率）折算成当前价值，从而计算出投资项目的净现值（NPV）或内部收益率（IRR）。
/// ### 现金折现率的含义和计算
/// **现金折现率**可以理解为未来现金流转化为当前价值时使用的利率。它反映了资金的时间价值和投资风险，即一单位货币在未来的价值不如现在，投资需要承担的风险越大，要求的现金折现率就越高。
/// 计算公式如下：
/// \[ PV = \frac{CF_1}{(1+r)^1} + \frac{CF_2}{(1+r)^2} + \cdots + \frac{CF_n}{(1+r)^n} \]
/// - \(PV\) 表示净现值（Present Value）；
/// - \(CF_n\) 表示第 n 年的现金流（Cash Flow）；
/// - \(r\) 表示现金折现率；
/// - \(n\) 表示期数。
/// ### 现金折现率的作用
/// - **投资评估**：通过计算投资项目的净现值（NPV）或内部收益率（IRR），来评估项目的盈利能力和风险，支持投资决策。
/// - **企业价值评估**：通过估算企业未来的自由现金流，并使用合适的折现率将这些现金流折现到现在，可以估算出企业的价值。
/// - **财务分析**：分析不同的投资方案或财务策略，比较它们的净现值或内部收益率，以确定最佳选择。
/// ### 现金折现率的确定
/// 现金折现率的确定通常基于市场利率、投资的风险程度以及投资者的风险偏好。它可能包括无风险利率、预期通货膨胀率、特定行业的风险溢价以及企业特有的风险溢价等因素。
/// 总的来说，现金折现率是一种非常重要的财务分析工具，能够帮助投资者和管理者理解投资项目或企业价值的真实情况，做出更加明智的决策。
fn main() {
    // 读取用户输入的本金、折现率和时间期限
    let mut input = String::new();

    println!("折现计算器");

    // 提示用户输入本金金额
    print!("请输入本金金额：");
    io::stdout().flush().expect("刷新失败"); // 刷新标准输出流，确保立即显示
    io::stdin().read_line(&mut input).expect("读取失败");
    let principal: f64 = input.trim().parse().expect("无效输入");

    input.clear(); // 清空输入缓冲区，以便下一次使用

    // 提示用户输入折现率
    println!("请输入折现率(以小数形式)：");
    io::stdin().read_line(&mut input).expect("读取失败");
    let discount_rate: f64 = input.trim().parse().expect("无效输入");

    input.clear(); // 清空输入缓冲区，以便下一次使用

    // 提示用户输入时间期限
    print!("请输入时间期限(以年为单位)：");
    io::stdout().flush().expect("刷新失败"); // 刷新标准输出流，确保立即显示
    io::stdin().read_line(&mut input).expect("读取失败");
    let time_period: u32 = input.trim().parse().expect("无效输入");

    // 计算并显示结果
    let result = calculate_present_value(principal, discount_rate, time_period);
    println!("现值为：{:.2}", result);
}

fn calculate_present_value(principal: f64, discount_rate: f64, time_period: u32) -> f64 {
    if discount_rate < 0.0 {
        eprint!("\n错误：折现率不能为负数！ "); // '\n'为换行转义符号
        eprintln!("\n请提供有效的折现率。");
        std::process::exit(1);
    }

    if time_period == 0 {
        eprint!("\n错误：时间期限不能为零！ ");
        eprintln!("\n请提供有效的时间期限。");
        std::process::exit(1);
    }

    // 计算简单移动平均线(SMA)
    sma_calculate();

    principal / (1.0 + discount_rate).powi(time_period as i32)
}

fn sma_calculate() {
    // 假设这是一个包含股票价格的数组
    let stock_prices = [50.0, 52.0, 55.0, 60.0, 58.0, 62.0, 65.0, 70.0, 75.0, 80.0];

    // 计算简单移动平均线(SMA)
    let window_size = 5; // 移动平均窗口大小
    let mut sma_values: Vec<f64> = Vec::new();

    for i in 0..stock_prices.len() - window_size + 1 {
        let window = &stock_prices[i..i + window_size];
        let sum: f64 = window.iter().sum();
        let sma = sum / window_size as f64;
        sma_values.push(sma);
    }

    // 打印 SMA 值
    println!("简单移动平均线(SMA):");
    for (i, sma) in sma_values.iter().enumerate() {
        println!("Day {}: {:.2}", i + window_size, sma);
    }
}
