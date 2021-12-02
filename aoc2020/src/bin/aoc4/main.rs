use std::collections::HashSet;
use regex::Regex;

fn main() { 
    println!("Answer to part 1: {}", part1(include_str!("input.txt")));
    println!("Answer to part 2: {}", part2(include_str!("input.txt")));
}

fn part1(inp:&str) -> i64 {
    let valid:HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
    let valid2:HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"].iter().cloned().collect();
    inp.split("\n\n")
        .map(|block| {
            let block = block.replace("\n", " ");
            let spl = block.split_whitespace();
            let mut kb:Vec<&str> = Vec::new();
            for k in spl {
                let kspl:Vec<&str> = k.split(":").collect();
                kb.push(kspl[0]);
            };
            let comp:HashSet<&str> = kb.iter().cloned().collect();
            if comp == valid || comp == valid2 {
                1
            } else {
                // println!("{:?}", comp); 
                0
            }
        })
        .sum() 
}

// not from me, off of reddit
fn part2(inp:&str) -> i64 {
    let lines: Vec<String> = inp.split("\n\n").map(|l| l.parse().unwrap()).collect();

    let byr_re = Regex::new(r"byr:(19[2-9][0-9]|200[0-2])\b").unwrap();
    let iyr_re = Regex::new(r"iyr:(201[0-9]|2020)\b").unwrap();
    let eyr_re = Regex::new(r"eyr:(202[0-9]|2030)\b").unwrap();
    let hgt_re = Regex::new(r"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\b").unwrap();
    let hcl_re = Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    let ecl_re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid_re = Regex::new(r"pid:[0-9]{9}\b").unwrap();

    let mut num_valid: i64 = 0;
        for i in 0..lines.len() {
            let s = (&lines[i]).to_lowercase();
            if  hgt_re.is_match(s.as_str()) &&
                eyr_re.is_match(s.as_str()) &&
                ecl_re.is_match(s.as_str()) &&
                pid_re.is_match(s.as_str()) &&
                hcl_re.is_match(s.as_str()) &&
                byr_re.is_match(s.as_str()) &&
                iyr_re.is_match(s.as_str())
                {
                    num_valid += 1;
                }
        }

    // println!("num_valid = {}", num_valid);
    num_valid
}
