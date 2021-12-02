use std::fs::File;
use std::io::{BufReader,BufRead};
use std::iter;

fn main() {
    let coll: Vec<i32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let total:i32 = coll.into_iter()
        .map(need)
        .flat_map(need_successors)
        .sum();
    println!("{}", total);
}

fn need(w: i32) -> i32 {
    0.max((w / 3) - 2)
}

fn need_successors(w: i32) -> impl Iterator<Item = i32> {
    iter::successors(Some(w), |&w| {
        if w == 0 { None } else { Some(need(w)) }
    })
}
