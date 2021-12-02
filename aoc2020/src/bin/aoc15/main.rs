// https://github.com/trinnguyen/advent-of-code-2020/blob/master/day15/src/main.rs
use std::collections::HashMap;

fn main() {
    println!("Part 1 Solution: {}", part1());
    println!("Part 2 Solution: {}", part2());
}

fn parse_input() -> Vec<u32> {
  include_str!("input.txt")
      .lines()
      .next()
      .unwrap()
      .split(",")
      .map(|i| { i.parse::<u32>().unwrap() })
      .collect()
}

fn part1() -> u32 {
    // println!("{:?}", parse_input());
    //let data:Vec<i32> = parse_input();
    //let mut spoken: HashMap<&i64, Vec<i64>> = HashMap::new();
    //for i in data {
    //    spoken.insert(i, Vec::new()); 
    //}
    //let mut turn:i64 = 0;
    //let mut idx:usize = 0;
    //while turn != 2020 {
    //    turn = turn + 1;
    //    if idx > (data.len() - 1) {
    //        idx = 0;
    //    } else {
    //        idx = idx + 1;
    //    }
    //    let key = data[idx]; 
    //    let v:Vec<i64> = spoken.get(key).unwrap();
    //    v.push(turn);
    //    spoken.insert(key, v);
    //    //println!("{}", turn);
    //}
    //println!("{:?}", spoken);
    let data:Vec<u32> = parse_input();
    calc_number(&data, 2020)
    //0
} 

fn part2() -> u32 {
    let data:Vec<u32> = parse_input();
    calc_number(&data, 30000000)
}

fn calc_number(data: &Vec<u32>, pos: usize) -> u32 {
    let mut last_num = 0;
    let mut turn: usize = 0;
    let mut map: HashMap<u32, (usize, usize)> = HashMap::new();

    while turn < pos {
        turn = turn + 1;

        last_num = match data.get(turn - 1) {
            Some(v) => *v,
            None => {
                // check last number
                match map.get(&last_num) {
                    Some((idx_fst, idx_snd)) => {
                        // it was spoken only once (last turn was its first time)
                        if *idx_snd == 0 {
                            0
                        } else {
                            // subtract 
                            (idx_snd - idx_fst) as u32
                        }
                    },
                    None => panic!("unexpected error") // never catch
                }
            }
        };

        // update map
        let new_pair: (usize, usize) = match map.get(&last_num) {
            Some((idx_fst, 0)) => (*idx_fst, turn),
            Some((_, idx_snd)) => (*idx_snd, turn),
            None => (turn, 0)
        };
        map.insert(last_num, new_pair);
    }

    last_num
}
