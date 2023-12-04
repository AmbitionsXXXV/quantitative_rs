// 在 main.rs
mod convert;
use polars::io::{csv::CsvReader, SerReader};

// const CSV_PATH: &str = "./588460.SH.csv";
const NEW_CSV_PATH: &str = "./588460_utf8.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = CsvReader::from_path(NEW_CSV_PATH)?.finish()?;

    println!("{:?}", df);

    Ok(())
}
