use std::collections::BTreeSet;
use std::collections::HashSet;

fn main() { 
    println!("Answer to part 1: {}", part1(include_str!("input.txt")));
    println!("Answer to part 2: {}", part2(include_str!("input.txt")));
}

fn part1(inp:&str) -> i64 {
    inp.split("\n\n")
        .map(|b| {
            let mut affirm = BTreeSet::new();
            let bc = b.chars();
            for c in bc {
                if c != '\n' {
                    affirm.insert(c);
                }
            }
            let mut res:i64 = 0;
            for _ in affirm.iter() {
                res = res + 1
            }
            res
        })
        .sum()
}

fn part2(inp:&str) -> i64 {
    inp.trim_end()
        .split("\n\n")
        .map(|block|{
            let indv = block.split_ascii_whitespace()
                .map(|p| p.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            indv.iter()
                .fold(indv[0].clone(), |all_yes, persons_answers| {
                    all_yes.intersection(persons_answers).cloned().collect()
                })
                .len() as i64
        })
        .sum()
    //let blocks:Vec<&str> = inp.split("\n\n").collect();
    //for group in blocks {
    //    let indv:Vec<&str> = group.split("\n").collect();
    //    let mut last:BTreeSet<char> = BTreeSet::new();
    //    // let mut curr:BTreeSet<char>;
    //    for i in indv {
    //        let mut curr:BTreeSet<char> = BTreeSet::new(); 
    //        let ic:Vec<char> = i.chars().collect();
    //        for c in ic {
    //            curr.insert(c);
    //        }
    //        let res: Vec<_> = last.intersection(&curr).cloned().collect();
    //        last = curr; //.clone();
    //        println!("{:?}", res);
    //    }
    //    //println!("{:?}", last)
    //}
    //0
}
