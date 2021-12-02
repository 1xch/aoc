
extern crate num_complex;
extern crate ordered_float;

use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Result};
use ordered_float::OrderedFloat;
use num_complex::Complex;

type Theta = OrderedFloat<f32>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn theta(&self, other: &Self) -> Theta {
        let (_, theta) = Complex::new((other.x - self.x) as f32, (self.y - other.y) as f32).to_polar();
        OrderedFloat((if theta <= 0f32 {
            theta.abs()
        } else {
            2f32 * std::f32::consts::PI - theta
        } + 0.5 * std::f32::consts::PI) % (2f32 * std::f32::consts::PI))
    }
}

fn read_points(path: &str) -> Result<Vec<Point>> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    Ok(f.lines()
     .enumerate()
     .flat_map(|(y, line)| 
        line.unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(x, _)| Point {
                x: x as i64,
                y: y as i64,
            }).collect::<Vec<Point>>()
     ).collect::<Vec<Point>>())
}

pub fn part1(path: &str) -> Result<i64> {
    let points = read_points(path)?;

    Ok(0)
}

pub fn part2(path: &str) -> Result<i64> {
    let points = read_points(path)?;

    let mut max_station: Point = Point{x: 0, y: 0};
    let mut max_hits: Vec<Point> = Vec::new();

    for station in &points {
        let mut seen: HashSet<Theta> = HashSet::new();
        let mut hits: Vec<Point> = Vec::new();

        for target in &points {
            if station != target {
                if seen.insert(station.theta(&target)) {
                    hits.push(target.clone());
                }
            }
        }

        if max_hits.len() < hits.len() {
            max_hits = hits;
            max_station = station.clone();
        }
    }

    let sorted_hits = &{
        max_hits.sort_by(|a, b| max_station.theta(a).cmp(&max_station.theta(b)));
        max_hits
    };

    let nth = sorted_hits.into_iter().nth(199).unwrap();

    Ok(nth.x * 100 + nth.y)
}

fn main() {
    //part1("input.txt");
    println!("{:?}", part2("input.txt"));
}
