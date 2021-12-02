use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part 1 Solution: {}", part1());
    println!("Part 2 Solution: {}", part2((parse_input()).1));
}

fn parse_input() -> (i64, Vec<String>) {
    let file = File::open("input.txt").expect("Couldn't read file");
    let mut input = BufReader::new(file).lines();
    
    let time_you_want_to_leave = input.nth(0).unwrap().unwrap().parse::<i64>().unwrap();
    let bus_schedules = input
        .next()
        .unwrap() // Option
        .unwrap() // Result
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    (time_you_want_to_leave, bus_schedules)
}

fn part1() -> i64 {
    let (time, bus_schedules) = parse_input();

    let best_option = bus_schedules
        .iter()
        .map(|x| x.parse::<i64>())
        .filter_map(|x| x.ok())
        .map(|x| (x, ((time % x) - x).abs()))
        .min_by_key(|x| x.1)
        .unwrap();

    best_option.0 * best_option.1
}

fn part2(buses: Vec<String>) -> i64 {
    let mut buses: Vec<(i64, i64)> = buses
        .iter()
        .enumerate() // Enumerate to get the # of minute offset for each bus
        .filter(|&(_, bus)| bus != "x")
        .map(|(index, bus)| (index as i64, bus.parse().unwrap()))
        .collect();

    buses.sort_by_key(|&(_, bus)| bus); // Sorting isn't necessary, but makes it faster

    // I initially stole this solution, but improved upon it (with for_each and better
    // variable names).It involves Chinese Remainder Theorem and search-by-sievin,
    let mut possible_solution = 0;
    let mut least_common_denominator = 1;
    buses.into_iter().for_each(|(minute_offset, bus)| {
        while (possible_solution + minute_offset) % bus != 0 {
            possible_solution += least_common_denominator;
        }
        least_common_denominator *= bus;
    });
    possible_solution
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! string_vec {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part2(string_vec!["17", "x", "13", "19"]), 3417);
        assert_eq!(part2(string_vec!["67", "7", "59", "61"]), 754018);
        assert_eq!(part2(string_vec!["67", "x", "7", "59", "61"]), 779210);
        assert_eq!(part2(string_vec!["67", "7", "x", "59", "61"]), 1261476);
        assert_eq!(part2(string_vec!["1789", "37", "47", "1889"]), 1202161486);
    }
}
