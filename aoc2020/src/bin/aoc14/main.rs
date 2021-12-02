// https://github.com/ropewalker/advent_of_code_2020/blob/master/src/day14.rs
use std::collections::HashMap;

const WORD_SIZE: usize = 36;

#[derive(Copy, Clone)]
struct BitMask {
    and_mask: u64,
    or_mask: u64,
}

impl Default for BitMask {
    fn default() -> Self {
        Self {
            and_mask: 2_u64.pow(WORD_SIZE as u32) - 1,
            or_mask: 0,
        }
    }
}

enum Instruction {
    Mask(BitMask),
    Mem(u64, u64),
}

fn parse_mask(mask_str: &str) -> BitMask {
    BitMask {
        and_mask: u64::from_str_radix(&mask_str.replace("X", "1"), 2).unwrap(),
        or_mask: u64::from_str_radix(&mask_str.replace("X", "0"), 2).unwrap(),
    }
}

// #[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" = ");
            let first = split.next().unwrap();

            match first {
                "mask" => Instruction::Mask(parse_mask(split.next().unwrap())),
                _ => {
                    let address = first[4..first.len() - 1].parse::<u64>().unwrap();
                    let value = split.next().unwrap().parse::<u64>().unwrap();
                    Instruction::Mem(address, value)
                }
            }
        })
        .collect()
}

// #[aoc(day14, part1)]
fn part1(instructions: &[Instruction]) -> u64 {
    let mut current_mask = BitMask::default();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => current_mask = *mask,
            Instruction::Mem(address, value) => {
                memory.insert(
                    *address,
                    value & current_mask.and_mask | current_mask.or_mask,
                );
            }
        }
    }

    memory.values().sum()
}

// #[aoc(day14, part2)]
fn part2(instructions: &[Instruction]) -> u64 {
    let mut current_mask = BitMask::default();
    let mut x_mask = 0;
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                current_mask = *mask;
                x_mask = current_mask.and_mask ^ current_mask.or_mask
            }
            Instruction::Mem(address, value) => {
                let mut address_base = address | current_mask.or_mask;
                let mut x_mask = x_mask;

                let mut addresses: Vec<u64> = Vec::new();

                for i in 0..WORD_SIZE {
                    let a = address_base % 2;
                    let x = x_mask % 2;

                    addresses = if x == 1 {
                        if addresses.is_empty() {
                            vec![0, 1]
                        } else {
                            addresses
                                .iter()
                                .map(|address| vec![address + 2_u64.pow(i as u32), *address])
                                .flatten()
                                .collect()
                        }
                    } else if addresses.is_empty() {
                        vec![a]
                    } else {
                        addresses
                            .iter()
                            .map(|address| address + a * 2_u64.pow(i as u32))
                            .collect()
                    };

                    address_base >>= 1;
                    x_mask >>= 1;
                }

                for address in addresses {
                    memory.insert(address, *value);
                }
            }
        }
    }

    memory.values().sum()
}

fn main() {
    println!("Part 1 Solution: {}", part1(&parse_input(include_str!("input.txt"))));
    println!("Part 2 Solution: {}", part2(&parse_input(include_str!("input.txt"))));
}

// use std::collections::HashMap;
// 
// #[derive(Debug)]
// enum Ins {
//     No = -1,
//     Mask = 0,
//     Memory = 1,
// }
// 
// #[derive(Debug)]
// struct Instruction {
//     ins: Ins,
//     mask: Vec<char>,
//     location: i64,
//     data: i64,
// }
// 
// impl Instruction {
//     fn new(inp: &str) -> Instruction {
//         Instruction{
//             ins: Ins::No,
//             mask: Vec::new(),
//             location: 0,
//             data: 0,
//         }
//     }
// }
// 
// #[derive(Debug)]
// struct Machine {
//     current_mask: Vec<char>, 
//     memory: HashMap<i64, i64>,
//     instructions: Vec<Instruction>,
// }
// 
// impl Machine {
//     fn new() -> Machine {
//         Machine{
//             current_mask: Vec::new(),
//             memory: HashMap::new(),
//             instructions: Vec::new(),
//         }
//     }
// 
//     fn parse(mut self, inp: &str) {
//         inp.lines()
//             .for_each(|l| {
//                 let i: Instruction = Instruction::new(l);
//                 self.instructions.push(i)
//             });
//     }
// }
// 
// fn part1(inp: &str) -> i64 {
//     let m:Machine = Machine::new();
//     m.parse(inp);
//     println!("{:?}", m);
//     0
// }
// 
// fn part2(_inp: &str) -> i64 {
//     0
// }
