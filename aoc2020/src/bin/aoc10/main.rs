// from https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions/ /u/karjonas
use std::collections::HashMap;

fn main() {
    let numbers = parse_input(include_str!("input.txt"));
    println!("Answer to part 1: {}", part1(&numbers));
    println!("Answer to part 2: {}", part2(&numbers));
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut list: Vec<usize> = input.lines().map(|v| v.parse::<usize>().unwrap()).collect();
    list.insert(0, 0);
    list.sort_unstable();
    let joltage_max = list.last().unwrap() + 3;
    list.push(joltage_max);
    return list;
}


fn part1(numbers: &Vec<usize>) -> usize {
    let mut num_one = 0;
    let mut num_three = 0;

    let mut curr_adapter = 0;
    for number in numbers {
        let diff = number - curr_adapter;
        if diff == 1 {
            num_one += 1;
        } else if diff == 3 {
            num_three += 1;
        }
        curr_adapter = *number;
    }

    return num_one * num_three;
}

fn part2(numbers:&Vec<usize>) -> usize {
    let final_number = *numbers.last().unwrap();
    let mut hits: HashMap<usize, usize> = HashMap::new();

    hits.insert(numbers[0], 1);

    for idx in numbers {
        let curr_hits = *hits.get(&idx).unwrap();

        for step in 1..4 {
            let idx_next = idx + step;
            if numbers.contains(&idx_next) {
                *hits.entry(idx_next).or_insert(0) += curr_hits;
            }
        }
    }

    return *hits.get(&final_number).unwrap();
}
