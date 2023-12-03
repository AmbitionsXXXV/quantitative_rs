use csv::Reader;
use encoding_rs::GBK;
use std::fs::read;
use std::io::Cursor;

const CSV_PATH: &str = "./588800.SH.csv";

fn main() {
    // 以二进制格式读取整个文件内容
    let data = read(CSV_PATH).expect("无法读取文件");

    // 将GBK编码的数据转换为UTF-8
    let (cow, _, _) = GBK.decode(&data);

    // 创建一个内存中的Cursor，以便csv库可以从中读取
    let cursor = Cursor::new(cow.as_bytes());

    // 使用Cursor创建CSV读取器
    let mut rdr = Reader::from_reader(cursor);

    // 逐行读取CSV
    for sr in rdr.records().flatten() {
        sr.into_iter().for_each(|c| println!("{}", c))
    }
}
