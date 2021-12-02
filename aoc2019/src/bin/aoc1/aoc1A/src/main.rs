use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let result: i32 = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse::<i32>().unwrap())
        .map(|x| (x/3)-2)
        .sum();
    println!("{}", result);
}
