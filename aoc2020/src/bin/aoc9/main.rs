use itertools::Itertools;

fn main() {
    println!("Answer to part 1: {}", part1(include_str!("input.txt")));
    println!("Answer to part 2: {}", part2(include_str!("input.txt")));
}

//fn parse(inp:&str) -> Vec<i64> {
//    inp.lines()
//    .map(|l| { l.parse::<i64>().unwrap() })
//    .collect()
//}
//
//fn block(begin:usize, end:usize, d:&Vec<i64>) -> &[i64] {
//    &d[begin..end]
//}
//
//fn is_valid(n:i64, p:&[i64]) -> bool {
//    let mut res:Vec<i64> = Vec::new();
//    p.iter()
//        .for_each( |i| {
//            let c = p.to_vec();
//            for ii in c {
//                res.push(i+ii);         
//            }
//        });
//    for r in &res {
//        if r == &n {
//            return true
//        }
//    }
//    println!("{:?} {:?}", n, p);
//    false
//}

fn part1(inp:&str) -> usize {
    //let data:Vec<i64> = parse(inp);
    //let mut idx:usize = 25;
    //let mut cont:bool = true;
    //let mut ret:i64 = 0;
    //while cont {
    //    let pre = block(idx-25,idx-1,&data); 
    //    //println!("{:?}", idx);
    //    let f:i64 = data[idx];
    //    if !is_valid(f, pre) {
    //        ret = f;
    //        return ret
    //    }
    //    idx = idx + 1;
    //    if idx >= data.len() {
    //        cont = false;
    //    }
    //}
    let data: Vec<usize> = inp.lines().map(|n| n.parse().unwrap()).collect();
    find_invalid_number(&data, 25).unwrap()
}

fn part2(inp:&str) -> usize {
    let data: Vec<usize> = inp.lines().map(|n| n.parse().unwrap()).collect();
    let invalid_number = find_invalid_number(&data, 25).unwrap();
    find_weakness(&data, invalid_number).unwrap() 
}

fn find_invalid_number(data: &Vec<usize>, window_size: usize) -> Option<usize> {
    data.windows(window_size + 1)
        .find(|v| {
            v[..(window_size)]
                .iter()
                .combinations(2)
                .all(|c| v[window_size] != c.iter().copied().sum())
        })
        .map(|v| v[window_size])
}

fn find_weakness(data: &Vec<usize>, invalid_number: usize) -> Option<usize> {
    for i in 2..=data.len() {
        if let Some(v) = data
            .windows(i)
            .find(|w| invalid_number == w.iter().sum())
        {
            return Some(v.iter().max().unwrap() + v.iter().min().unwrap());
        }
    }
    None
}
