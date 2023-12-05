use sqlx::{mysql::MySqlPoolOptions, Error, Row};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://<username>:<password>@localhost:<port>/<db_name>")
        .await?;

    // 其他逻辑

    let rows = sqlx::query("SELECT tel FROM aimyon.user")
        .fetch_all(&pool)
        .await?;

    rows.iter().for_each(|row| {
        let tel: String = row.get("tel");
        println!("电话号码: {}", tel);
    });

    Ok(())
}
