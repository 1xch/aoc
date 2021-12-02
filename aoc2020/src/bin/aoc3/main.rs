//  https://github.com/Lakret/aoc2020/blob/master/src/d3.rs
// use std::collections::HashSet;
// 
// fn main() { 
//     println!("Answer to part 1: {:?}", part1(include_str!("input.txt")));
//     println!("Answer to part 2: {:?}", part2(include_str!("input.txt")));
// }
// 
// pub fn part1(input: &str) -> Option<i64> {
//   let tree_map = TreeMap::parse(input);
//   let trees_count = tree_map.tree_count_on_slope(3, 1);
//   Some(trees_count as i64)
// }
// 
// pub fn part2(input: &str) -> Option<i64> {
//   let tree_map = TreeMap::parse(input);
// 
//   let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
//   let answer = slopes.into_iter().fold(1, |answer, (right, down)| {
//     answer * tree_map.tree_count_on_slope(right, down)
//   });
// 
//   Some(answer as i64)
// }
// 
// #[derive(Debug)]
// struct TreeMap {
//   trees: HashSet<(usize, usize)>,
//   width: usize,
//   height: usize,
// }
// 
// impl TreeMap {
//   pub fn parse(input: &str) -> TreeMap {
//     let mut trees = HashSet::new();
//     let mut width = None;
//     let mut height = None;
// 
//     // y goes from top to bottom, starting from 0
//     for (y, line) in input.trim_end().split('\n').enumerate() {
//       if width == None {
//         width = Some(line.len());
//       }
//       height = Some(y + 1);
// 
//       for (x, char) in line.chars().enumerate() {
//         // x goes from left to right, starting from 0
//         if char == '#' {
//           trees.insert((x, y));
//         }
//       }
//     }
// 
//     let width = width.expect("empty input is not valid");
//     let height = height.expect("empty input is not valid");
//     TreeMap {
//       trees,
//       width,
//       height,
//     }
//   }
// 
//   pub fn is_tree(&self, (x, y): (usize, usize)) -> bool {
//     // emulate copying everything to the right infinite amount of times
//     let x = x % self.width;
// 
//     self.trees.contains(&(x, y))
//   }
// 
//   pub fn move_with_slope(
//     &self,
//     (x, y): (usize, usize),
//     right: usize,
//     down: usize,
//   ) -> Option<(usize, usize)> {
//     let x = x + right;
//     let y = y + down;
// 
//     if y < self.height {
//       Some((x, y))
//     } else {
//       None
//     }
//   }
// 
//   pub fn tree_count_on_slope(&self, right: usize, down: usize) -> usize {
//     let mut trees_count = 0;
//     let mut position = Some((0, 0));
//     while let Some(curr_position) = position {
//       if self.is_tree(curr_position) {
//         trees_count += 1;
//       }
// 
//       position = self.move_with_slope(curr_position, right, down);
//     }
// 
//     trees_count
//   }
// }

// my solution -- it is wrong because I didn't split the line data properly -- inserts quotation marks ("")
use std::fmt;

fn main() { 
    println!("Answer to part 1: {}", part1());
    println!("Answer to part 2: {}", part2());
}

fn part1() -> i64 {
    let mut d = open();
    let mut cont = true;
    while cont {
        cont = d.hook();  
        // println!("{}", d.current_obstacle());
        // d.current_location()
    } 
    d.tree
}

fn part2() -> i64 {
    0
}

#[derive(Debug)]
struct Data {
    pub lims: Vec<i64>,
    pub curr: Vec<i64>,
    pub data: Vec<Vec<String>>,
    pub tree: i64, 
}

static RIGHT_SHIFT: i64 = 3;
static DOWN_SHIFT: i64 = 1;

impl Data {
    pub fn hook(&mut self) -> bool {
        // self.current_location();
        let next_y = self.curr[1] + DOWN_SHIFT;
        if next_y > self.lims[1] {
            return false
        }        
        let mut next_x = self.curr[0] + RIGHT_SHIFT;
        if next_x > self.lims[0] {
            next_x = next_x - self.lims[0];
        }
        
        self.curr[0] = next_x;
        self.curr[1] = next_y;
        
        println!("curr[{},{}] {}", self.curr[0], self.curr[1], self.current_obstacle());

        true 
    }

    pub fn current_obstacle(&self) -> String {
        let xloc:usize = self.curr[0] as usize;
        let yloc:usize = self.curr[1] as usize;
        let y = &self.data[yloc];
        let ret = &y[xloc];
        ret.to_string()
    }

    pub fn current_location(&mut self) {
        let co = self.current_obstacle();
        if co == "#" {
            self.tree = self.tree + 1;
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "curr: [{}, {}]\nlims: [{}, {}]", self.curr[0], self.curr[1], self.lims[0], self.lims[1])
    }
}

fn open() -> Data {
    let mut xlim = 0;
    let mut d:Vec<Vec<String>> = Vec::new();
    include_str!("input.txt")
        .lines()
        .for_each(|l| {
            let spl = l.split("");
            let mut x:Vec<String> = Vec::new();
            for i in spl {  
                x.push(i.to_string());
            }
            println!("{:?}", x);
            if xlim == 0 {
                xlim = (x.len() - 1) as i64;
            }
            d.push(x); 
        });
    let mut lims: Vec<i64> = Vec::new();
    lims.push(xlim);
    let ylim = d.len() as i64;
    lims.push(ylim - 1);
    let mut curr: Vec<i64> = Vec::new();
    curr.push(0);
    curr.push(0);
    let ret:Data = Data{
        lims: lims,
        curr: curr,
        data: d,
        tree: 0,
    };
    ret
}
