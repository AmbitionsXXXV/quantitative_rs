# This is a quantitative with rust

## How to run

```bash
cargo run -p <members name>
```

### convert_csv

convert gbk CSV to utf8 parquet

add `convert_csv` to your `Cargo.toml`

```toml
[dependencies]
convert_csv = { path = "../convert_csv" }
```

```rs
use convert_csv::{Converter, GbkToUtf8Converter};

const CSV_PATH: &str = "./588460.SH.csv";
const NEW_CSV_PATH: &str = "./588460_utf8.csv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter;
    converter.convert(CSV_PATH, NEW_CSV_PATH)?;

    Ok(())
}
```
