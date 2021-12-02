use std::collections::BTreeSet;

fn main() { 
    println!("Answer to part 1: {}", part1(include_str!("input.txt")));
    println!("Answer to part 2: {}", part2(include_str!("input.txt")));
}

fn read_ids(inp: &str) -> BTreeSet<u16> {
    let mut ids = BTreeSet::new();

    for line in inp.lines() {
        // id = col * 8 + row, which is the same as col << 3 + row
        // we can just skip that and parse the entire line as u16
        let id = line.chars().fold(0, |acc, elem| {
            (acc << 1) + if elem == 'B' || elem == 'R' { 1 } else { 0 }
        });
        ids.insert(id);
    }
    ids
}


fn part1(inp:&str) -> u16 {
    let ids = read_ids(inp);
    *ids.iter().next_back().unwrap()
    //let mut ret:u32 = 0;
    //let lines = inp.lines();
    //for l in lines {
    //    let r:u32 = row(&l[0..7]); 
    //    let c:u32 = column(&l[7..10]);
    //    let id:u32 = id(r,c);
    //    if id >= ret {
    //        ret = id;
    //        println!("{}", ret)
    //    }
    //    // println!("{} {} {}", r, c, id)
    //}
    //ret
}

fn part2(inp:&str) -> u16 { 
    let ids = read_ids(inp);
    for id in ids.iter() {
        // if the next id is not in the set and the next next id is, we found our gap
        if !ids.contains(&(id + 1)) && ids.contains(&(id + 2)) {
            return id + 1;
        }
    }
    // We should return an Option<u16>, but input is well formed and there is always an answer
    // so we should never reach this point, but panic if we do.
    unreachable!();
}

//fn row(inp:&str) -> u32 {
//    let mut n:Vec<u32> = Vec::new(); 
//    for i in inp.chars() {
//        match i {
//            'F' => n.push(0),
//            'B' => n.push(1),
//            _ => (), 
//        }          
//    }
//    n.iter().fold((0,1),|(acc,mul),&bit|(acc+(mul*(1&bit as u32)),mul.wrapping_add(mul))).0
//}

//fn column(inp:&str) -> u32 {
//    let mut n:Vec<u32> = Vec::new(); 
//    for i in inp.chars() {
//        match i {
//            'R' => n.push(0),
//            'L' => n.push(1),
//            _ => (), 
//        }          
//    }
//    n.iter().fold((0,1),|(acc,mul),&bit|(acc+(mul*(1&bit as u32)),mul.wrapping_add(mul))).0
//}

//fn id(r:u32, c:u32) -> u32 {
//    (r * 8) + c
//}
