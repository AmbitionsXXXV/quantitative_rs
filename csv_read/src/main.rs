use csv::Reader;
use encoding_rs::GBK;
use std::fs::read;
use std::io::Cursor;

// 因为相对路径是根据执行目录来计算的，所以这里的 csv 文件路径是相对于执行目录的，放在子项目根目录读取不到，除非在该子目录下执行 cargo run
// const CSV_PATH: &str = "./588800.SH.csv";
// 要使用相对路径读取子项目的 csv 文件，则需要使用如下相对路径
const CSV_PATH: &str = "./csv_read/588460.SH.csv";

fn main() {
    // 以二进制格式读取整个文件内容
    let data = read(CSV_PATH).expect("无法读取文件");

    // 将 GBK 编码的数据转换为 UTF-8
    let (cow, _, _) = GBK.decode(&data);

    // 创建一个内存中的 Cursor，以便 csv 库可以从中读取
    let cursor = Cursor::new(cow.as_bytes());

    // 使用 Cursor 创建 CSV 读取器
    let mut rdr = Reader::from_reader(cursor);

    // 逐行读取 CSV
    for sr in rdr.records().flatten() {
        // 打印第一列数据
        println!("row: {:?}", sr.get(0).unwrap());

        // 打印所有列数据
        // sr.into_iter().for_each(|c| println!("{}", c))
        // 使用只读迭代器
        // sr.iter().for_each(|c| println!("{}", c))

        // 使用 map 遍历
        let new_st = sr.iter().map(|c| c.to_string()).collect::<Vec<String>>();

        // 最终打印结果为每次循环打印一行数据
        println!("{:?}", new_st)
    }
}
