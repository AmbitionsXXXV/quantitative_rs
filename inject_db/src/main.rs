// 导入所需的库和模块
use polars::prelude::*;
use sqlx::{mysql::MySqlPoolOptions, Error, Row};
use with_polars_lazy::filter_csv_lazy;

// 定义将要读取的 CSV 文件路径
const NEW_CSV_PATH: &str = "./2023-12-03_new.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 LazyCsvReader 从文件路径读取 CSV 数据，并将其转换为 DataFrame
    let df = LazyCsvReader::new(NEW_CSV_PATH) // 创建一个新的 LazyCsvReader 实例，指定 CSV 文件路径
        .has_header(true) // 设置 CSV 文件包含表头
        .finish()? // 完成配置并初始化读取器
        .collect()?; // 收集数据到 DataFrame

    // 使用自定义函数 filter_csv_lazy 对 DataFrame 进行过滤
    let df = filter_csv_lazy(df, "^sh6002")?; // 过滤 DataFrame，保留符合正则表达式 "^sh6002" 的行

    // 打印过滤后的 DataFrame
    println!("{:?}", df);

    // 创建一个新的 Tokio 运行时
    let rt = tokio::runtime::Runtime::new()?; // 创建并初始化一个新的 Tokio 运行时

    // 创建一个本地任务集
    let local_set = tokio::task::LocalSet::new(); // 创建一个新的本地任务集

    // 在本地任务集上启动 insert 异步函数
    let handle = local_set.spawn_local(async {
        insert().await.unwrap(); // 使用 spawn_local 在本地任务集上启动 insert 异步函数
    });

    local_set.block_on(&rt, handle)?; // 在 Tokio 运行时上执行并等待异步任务完成

    Ok(())
}

// 异步函数用于操作数据库
async fn insert() -> Result<(), Error> {
    // 创建并配置 MySQL 连接池
    let pool = MySqlPoolOptions::new() // 创建一个新的 MySqlPoolOptions 实例
        .max_connections(10) // 设置连接池的最大连接数为 10
        .connect("mysql://<username>:<password>@localhost:<port>/<db_name>") // 连接到 MySQL 数据库，指定连接字符串
        .await?; // 等待异步连接操作完成

    // 执行 SQL 查询，从 rust_quant.users 表中获取数据
    let rows = sqlx::query("SELECT id, name, age FROM rust_quant.users")
        .fetch_all(&pool) // 向数据库发送查询并获取所有结果
        .await?; // 等待异步查询操作完成

    // 遍历查询结果并打印
    for row in rows {
        let id: i32 = row.get("id"); // 获取名为 'id' 的列的值
        let name: String = row.get("name"); // 获取名为 'name' 的列的值
        println!("ID: {}, 姓名: {}", id, name);
    }

    // 插入新的数据行到 users 表
    let new_row = sqlx::query("INSERT INTO users(name, age) VALUES (?, ?)")
        .bind("aimyon") // 绑定第一个参数 'aimyon' 到 SQL 查询
        .bind(28) // 绑定第二个参数 '28' 到 SQL 查询
        .execute(&pool) // 向数据库发送插入命令
        .await?; // 等待异步插入操作完成

    // 打印插入操作的结果
    println!("插入的行数: {}", new_row.rows_affected()); // 打印受影响的行数
    println!("插入的id: {}", new_row.last_insert_id()); // 打印插入的最后一个 ID

    Ok(())
}
