use sqlx::{mysql::MySqlPoolOptions, Error, Row};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://<username>:<password>@localhost:<port>/<db_name>")
        .await?;

    // 其他逻辑

    let rows = sqlx::query("SELECT id,name,age FROM rust_quant.users")
        .fetch_all(&pool)
        .await?;

    rows.iter().for_each(|row| {
        let id = row.get::<i32, _>("id");
        println!("电话号码: {}", id);
    });

    Ok(())
}
