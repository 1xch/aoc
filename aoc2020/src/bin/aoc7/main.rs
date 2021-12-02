// not my solution
// https://gist.github.com/Aehmlo/bed47dafd0e25b975bde474ceb48cdf1
use std::collections::HashMap;

fn main() { 
    println!("Answer to part 1: {}", part1(&parse(include_str!("input.txt"))));
    println!("Answer to part 2: {}", part2(&parse(include_str!("input.txt"))));
}

type Rules = HashMap<String, HashMap<String, usize>>;

fn parse(inp: &str) -> Rules {
    inp.lines()
        .map(|l| {
            let mut parts = l.split(" bags contain ");
            let color = parts.next().unwrap().to_string();
            let rules = parts
                .next()
                .unwrap()
                .split(", ")
                .filter_map(|element| {
                    let mut words = element.splitn(2, ' ');
                    let n = match words.next()? {
                        "no" => None,
                        n => n.parse::<usize>().ok(),
                    }?;
                    let inner = words.next()?.rsplitn(2, ' ').skip(1).next()?.to_string();
                    (inner, n).into()
                })
                .collect::<HashMap<String, usize>>();
            (color, rules)
        })
        .collect()
}

fn part1(rs: &Rules) -> usize {
    rs
        .keys()
        .filter(|color| can_reach_gold_bag(rs, color))
        .count()
}

fn can_reach_gold_bag(rules: &Rules, color: &str) -> bool {
    rules[color]
        .iter()
        .any(|(color, _)| color == "shiny gold" || can_reach_gold_bag(rules, color))
}

fn part2(rs: &Rules) -> usize {
    bag_count(rs, "shiny gold") - 1
}

fn bag_count(rules: &Rules, color: &str) -> usize {
    1_usize
        + rules[color]
            .iter()
            .map(|(color, count)| count * bag_count(rules, color))
            .sum::<usize>()
}
