extern crate array_tool;
extern crate itertools;

use array_tool::vec::Intersect;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

fn main() {
    let input: String = fs::read_to_string("input").expect("Something went wrong reading the file");
    let items: HashMap<String, String> = input
        .trim()
        .lines()
        .flat_map(|s| s.rsplit(")"))
        .map(|t| String::from(t))
        .tuples()
        .collect();

    let mut total_count: i32 = 0;
    let mut santa_path: Vec<String> = Vec::new();
    let mut you_path: Vec<String> = Vec::new();

    for planet in &items {
        let mut planet_count: i32 = 1;
        let mut parent: String = planet.1.to_string();
        let mut is_santa: bool = false;
        let mut is_you: bool = false;

        match planet.0.as_str() {
            "YOU" => {
                is_you = true;
                you_path.push(planet.0.to_string());
            }
            "SAN" => {
                is_santa = true;
                santa_path.push(planet.0.to_string());
            }
            _ => {}
        }

        while items.contains_key(&parent) {
            if is_you {
                you_path.push(parent.to_string())
            }
            if is_santa {
                santa_path.push(parent.to_string())
            }
            parent = items.get(&parent).unwrap().to_string();
            planet_count += 1;
        }
        total_count += planet_count;
    }
    println!("total orbital count (part 1): {}", total_count);

    let intersection = you_path.intersect(santa_path.clone());
    let santa_path_length = santa_path
        .iter()
        .position(|x| x == &intersection[0])
        .unwrap()
        - 1;
    let you_path_length = you_path
        .iter()
        .position(|x| x == &intersection[0])
        .unwrap() 
        - 1;
    let orbital_length = santa_path_length + you_path_length;

    println!(
        "santa_path_length (part 2): {} you_path_length: {} orbital length: {}",
        santa_path_length, you_path_length, orbital_length
    );
}
