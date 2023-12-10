use std::{env::Args, fmt::Write, io::BufWriter, path::Display};

#[tokio::main]
async fn main() {
    let mut args: Vec<u64> = std::env::args().filter_map(|s| s.parse().ok()).collect();
    assert!(args.len() == 1);

    let day = args.pop().unwrap();
    let filename = format!("inputs/day_{}.txt", &day);
    if std::fs::File::open(&filename).is_ok() {
        panic!("File {} already exists", &filename)
    };

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let input = reqwest::Client::new().get(&url).header("Cookie", "").send();
    let input = input.await.unwrap().bytes().await.unwrap();

    let mut file = BufWriter::new(std::fs::File::create(&filename).unwrap());
    std::io::Write::write_all(&mut file, &input).unwrap();

    println!("Saved file: {}", &filename);
    println!("{:?}", &input);
}
