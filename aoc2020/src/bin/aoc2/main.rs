use std::convert::TryInto;

fn main() { 
    println!("Answer to part 1: {}", part1());
    println!("Answer to part 2: {}", part2());
}

type Password<'a> = &'a str;

fn part1() -> i64 {
    include_str!("input.txt")
        .lines()
        .map(|l| {
            let spl:Vec<&str> = l.split_whitespace().collect();
            let p:Password = spl[2];
            let policy:char = spl[1].chars().next().unwrap();
            let mn_mx:Vec<i32> = spl[0]
                .split("-")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            let mn = mn_mx[0];
            let mx = mn_mx[1];
            let nlip = p.chars().filter(|c| *c == policy).count() as i32;
            if nlip >= mn && nlip <= mx {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

fn part2() -> i64 {
    include_str!("input.txt")
        .lines()
        .map(|l| {
            let spl:Vec<&str> = l.split_whitespace().collect();
            let p:Password = spl[2];
            let policy:char = spl[1].chars().next().unwrap();
            let mn_mx:Vec<i32> = spl[0]
                .split("-")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            fn split_by_pos(
                pos:usize,
                password:&Password,
                mn_mx:&Vec<i32>,
                policy:char,
            ) -> bool {
                password
                    .chars()
                    .skip((mn_mx[pos] - 1).try_into().unwrap())
                    .next()
                    .map_or(false, |val| val == policy)
            };
            let first_pos_matches = split_by_pos(0, &p, &mn_mx, policy);
            let second_pos_matches = split_by_pos(1, &p, &mn_mx, policy);
            if first_pos_matches ^ second_pos_matches {
                1
            } else {
                0
            }
        })
        .sum()
}
