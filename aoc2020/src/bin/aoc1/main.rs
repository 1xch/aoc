use aoc2020::input::parse_list;

static TARGET:i64 = 2020; 

fn main() {
    let inp: Vec<i64> = parse_list(include_str!("input.txt"), '\n');
    println!("Answer to part 1: {}", part1(&inp));
    println!("Answer to part 2: {}", part2(&inp));
}

fn part1(inp: &Vec<i64>) -> i64 {
    let mut idx = 0;
    let mut nf = true;
    let mut ret = 0;
    let body = &inp[0..];
    while nf {
        let head = inp[idx];
        for (vdx, val) in body.iter().enumerate() {
            if vdx != idx {
                let res:i64 = val + head;
                // println!("{} + {} = {}", head, val, res);
                if res == TARGET {
                    ret = val * head;
                    nf = false;
                }
            }
        }
        idx = idx + 1;
    }
    ret
}

fn part2(inp: &Vec<i64>) -> i64 {
    let mut nf = true;
    let mut ret = 0;
    let body = &inp;
    while nf {
        for (vdx, _) in body.iter().enumerate() {
            let h1 = body[vdx];
            for (vdx2, val2) in body.iter().enumerate() {
                if vdx != vdx2 {
                    let h2 = val2;
                    let inter = h1 + h2;
                    for (vdx3, val3) in body.iter().enumerate() {
                        if vdx3 != vdx2 && vdx3 != vdx {
                            let h3 = val3;
                            let fv = inter + h3; 
                            if fv == TARGET {
                                ret = h1 * h2 * h3;
                                nf = false;
                            }
                        }
                    }
                }
            }
        }
                
    }
    ret
}
