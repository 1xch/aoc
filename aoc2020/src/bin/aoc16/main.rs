// https://www.reddit.com/r/adventofcode/comments/ke2qp6/2020_day_16_solutions/ /u/bxmg
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("Part 1 Solution: {}", part1(include_str!("input.txt")));
    println!("Part 2 Solution: {}", part2(include_str!("input.txt")));
}

fn rule(s: &str) -> HashSet<usize> {
    let mut r: HashSet<usize> = HashSet::new();
    for p in s.split(" or ") {
        let ns: Vec<usize> = p.split("-").map(|s| s.parse().unwrap()).collect();
        for i in ns[0]..ns[1] + 1 {
            r.insert(i);
        }
    }
    r
}

fn rules(input: &str) -> HashMap<&str, HashSet<usize>> {
    let parts: &str = input.split("\n\n").nth(0).unwrap();
    parts
        .lines()
        .map(|s| {
            let p: Vec<&str> = s.split(": ").collect();
            (p[0], rule(p[1]))
        })
        .collect()
}

fn nearby(input: &str) -> Vec<Vec<usize>> {
    let s: &str = input.split("\n\n").nth(2).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    (lines[1..])
        .iter()
        .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn your(input: &str) -> Vec<usize> {
    let s: &str = input.split("\n\n").nth(1).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    lines[1].split(',').map(|n| n.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let all_rules = rules(&input);
    let all_nearby = nearby(&input);

    let all_valid_numbers = all_rules
        .values()
        .fold(HashSet::new(), |accl, s| accl.union(s).copied().collect());
    all_nearby.iter().fold(0, |accl, v| {
        accl + v
            .iter()
            .filter(|i| !all_valid_numbers.contains(i))
            .sum::<usize>()
    })
}

fn column(v: &Vec<Vec<usize>>, c: usize) -> HashSet<usize> {
    v.iter().map(|vv| vv[c]).collect()
}

fn part2(input: &str) -> usize {
    let all_rules = rules(&input);
    let all_nearby = nearby(&input);
    let your_ticket = your(&input);
    let fields = your_ticket.len();

    let all_valid_numbers = all_rules
        .values()
        .fold(HashSet::new(), |accl, s| accl.union(s).copied().collect());

    let mut valid_tickets: Vec<Vec<usize>> = all_nearby
        .into_iter()
        .filter(|v| v.iter().all(|i| all_valid_numbers.contains(i)))
        .collect();
    valid_tickets.push(your_ticket.clone());

    let ticket_columns: Vec<HashSet<usize>> =
        (0..fields).map(|c| column(&valid_tickets, c)).collect();

    let mut confirmed_rules: HashMap<&str, usize> = HashMap::new();
    while confirmed_rules.len() != fields {
        for (i, column_values) in ticket_columns.iter().enumerate() {
            let valid_rules: Vec<&str> = all_rules
                .iter()
                .filter(|(k, s)| {
                    !confirmed_rules.contains_key(*k) && column_values.difference(s).count() == 0
                })
                .map(|(k, _)| *k)
                .collect();
            if valid_rules.len() == 1 {
                confirmed_rules.insert(valid_rules[0], i);
            }
        }
    }

    confirmed_rules
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| *v)
        .map(|v| your_ticket[v])
        .fold(1, |accl, n| accl * n)
}

//use std::collections::HashMap;

//fn main() {
//    println!("Part 1 Solution: {}", part1(include_str!("input.txt")));
//    println!("Part 2 Solution: {}", part2());
//}
//
//fn parse_input(inp: &str) {
//    let blocks:Vec<&str> = inp.split("\n\n").collect(); 
//    // valid
//    let valid: &str = blocks[0];
//    // your ticket
//    let you: &str = blocks[1];
//    // nearby
//    let near: &str = blocks[2];
//
//    println!("{:?}\n\n{:?}\n\n{:?}\n", valid, you, near);
//}
//
//struct Item {
//    key: String,
//    value: Vec<(u32,u32)>,
//}
//
//fn parse_valid(inp: &str) -> Vec<Item> {
//    inp.split("\n")
//        .map( |i| {
//            Item{
//                key: parse_key(i),
//                // value: parse_value(i),
//            }
//        })
//        .collect()
//} 
//
//fn parse_key(inp: &str) -> String {
//    inp.split(":")
//    .next()
//    .unwrap()
//    .to_string()
//}
//
//fn parse_value(inp: &str) -> Vec<(u32,u32)> {
//    inp.split(":")
//    .last()
//    .unwrap()
//    .split("or")
//    .map(|s| {
//        s.split("-")
//        .map(|i| {  i.parse::<u32>().unwrap() })
//        .collect()
//    })
//    .collect()
//}
//
//fn part1(inp: &str) -> u32 {
//    parse_input(inp);
//    0
//}
//
//fn part2() -> u32 {
//    0
//}
