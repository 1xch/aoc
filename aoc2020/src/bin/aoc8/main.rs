// https://github.com/aldanor/aoc-2020/blob/master/rust/src/day08/mod.rs
use aoc2020::utils::*;

#[inline]
pub fn input() -> &'static [u8] {
    static INPUT: &[u8] = include_bytes!("input.txt");
    INPUT
}

#[inline]
pub fn part1(s: &[u8]) -> i16 {
    Runner::from_input(s).execute()
}

#[inline]
pub fn part2(s: &[u8]) -> i16 {
    Runner::from_input(s).find_bug_and_execute()
}

fn main() { 
    println!("Answer to part 1: {}", part1(input()));
    println!("Answer to part 2: {}", part2(input()));
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Op {
    Nop = 0,
    Jmp = 1,
    Acc = 2,
}

impl Default for Op {
    fn default() -> Self {
        Self::Nop
    }
}

impl Op {
    #[inline]
    pub fn invert(self) -> Self {
        match self {
            Self::Jmp => Self::Nop,
            Self::Acc => Self::Acc,
            Self::Nop => Self::Jmp,
        }
    }
}

type Pos = i16;

#[derive(Debug, Copy, Clone, Default)]
pub struct Cmd {
    op: Op,
    arg: i16,
}

impl Cmd {
    #[inline]
    pub fn execute(self, pos: &mut Pos, acc: &mut i16) {
        match self.op {
            Op::Jmp => *pos += self.arg,
            Op::Acc => {
                *pos += 1;
                *acc += self.arg
            }
            Op::Nop => *pos += 1,
        }
    }

    #[inline]
    pub fn invert(self) -> Self {
        Self {
            op: self.op.invert(),
            arg: self.arg,
        }
    }
}

#[inline]
fn parse_i16(s: &mut &[u8]) -> i16 {
    let neg = s.get_first() == b'-';
    *s = s.advance(1);
    let mut d = s.get_digit() as i16;
    *s = s.advance(1);
    let c1 = s.get_first().wrapping_sub(b'\n');
    if c1 != 0 {
        d = d * 10 + (c1.wrapping_sub(b'0' - b'\n') as i16);
        *s = s.advance(1);
        let c2 = s.get_first().wrapping_sub(b'\n');
        if c2 != 0 {
            d = d * 10 + (c2.wrapping_sub(b'0' - b'\n') as i16);
            *s = s.advance(1);
        }
    }
    if neg {
        -d
    } else {
        d
    }
}

impl Cmd {
    #[inline]
    pub fn try_parse(s: &mut &[u8]) -> Option<Self> {
        if s.len() > 1 {
            let op = match s.get_first() {
                b'j' => Op::Jmp,
                b'a' => Op::Acc,
                _ => Op::Nop,
            };
            *s = s.advance(4);
            let arg = parse_i16(s);
            *s = s.advance(1);
            Some(Cmd { op, arg })
        } else {
            None
        }
    }
}

pub const MAX_CMDS: usize = 1024;

const NULL: i16 = i16::MIN;

#[derive(Debug, Clone, Copy)]
pub enum CellState {}

#[derive(Debug, Copy, Clone)]
pub struct Runner {
    cmds: [Cmd; MAX_CMDS],
    n_cmds: usize,
}

impl Runner {
    pub fn from_input(mut s: &[u8]) -> Self {
        let mut cmds = [Cmd::default(); MAX_CMDS];
        let mut n_cmds = 0;
        while let Some(cmd) = Cmd::try_parse(&mut s) {
            cmds[n_cmds] = cmd;
            n_cmds += 1;
        }
        Self { cmds, n_cmds }
    }

    pub fn execute(&self) -> i16 {
        let (mut pos, mut acc) = (0, 0);
        let mut visited = [false; MAX_CMDS];
        loop {
            let v = unsafe { visited.get_unchecked_mut(pos as usize) };
            if *v {
                break acc;
            }
            unsafe { *self.cmds.get_unchecked(pos as usize) }.execute(&mut pos, &mut acc);
            *v = true;
        }
    }

    #[inline]
    pub fn find_bug_and_execute(&self) -> i16 {
        let mut seen = [false; MAX_CMDS];
        self.traverse_and_flip(0, 0, false, &mut seen)
    }

    #[inline]
    fn traverse_and_flip(
        &self,
        pos: Pos,
        acc: i16,
        flipped: bool,
        seen: &mut [bool; MAX_CMDS],
    ) -> i16 {
        let i = pos as usize;
        if pos < 0 || unsafe { *seen.get_unchecked(i) } {
            return NULL;
        } else if pos >= (self.n_cmds as Pos) {
            return acc;
        }
        unsafe { *seen.get_unchecked_mut(i as usize) = true };
        let cmd = unsafe { *self.cmds.get_unchecked(pos as usize) };
        {
            let (mut pos, mut acc) = (pos, acc);
            cmd.execute(&mut pos, &mut acc);
            let acc = self.traverse_and_flip(pos, acc, flipped, seen);
            if acc != NULL {
                return acc;
            }
        }
        if !flipped {
            let (mut pos, mut acc) = (pos, acc);
            cmd.invert().execute(&mut pos, &mut acc);
            let acc = self.traverse_and_flip(pos, acc, true, seen);
            if acc != NULL {
                return acc;
            }
        }
        NULL
    }
}

// me, only part of a solution
// problems with matching in impl Machine step()
// use std::collections::HashMap;

//#[derive(Debug)]
//struct Machine {
//    idx: i32,
//    accumulation: i32,
//    data: Vec<Instruction>,
//}
//
//impl Machine{
//    pub fn new(inp: &str) -> Machine {
//        Machine{
//            idx: 0,
//            accumulation: 0,
//            data: inp.lines()
//                .map(|line| {
//                    let spl:Vec<&str> = line.split(" ").collect();
//                    Instruction{
//                        key: spl[0].to_string(),
//                        value: spl[1].to_string().parse::<i32>().unwrap(),
//                        count: 0,
//                    } 
//                }).
//                collect(),
//        }
//    } 
//   
//    pub fn step(&mut self) -> i32 {
//        let idx: usize = self.idx as usize; 
//        let mut curr: Instruction = self.data.get(idx).unwrap();
//        let mut ret: i32 = 0;
//        match curr.key.as_ref() {
//           "acc" => (), // { ret = curr.value; self.incr(1) }, 
//           "jmp" => (), // self.incr(curr.value), 
//            _ => (),
//        }
//        curr.count = curr.count + 1;
//        ret
//    }
//
//    pub fn step_to_repeat(&mut self) -> i32 {
//        let mut cont: bool = true;
//        while cont {
//            let step:i32 = self.step();
//            println!("{}", step);
//            self.accumulation = self.accumulation + step;
//            //if self.iter != 1 {
//            //    cont = false;
//            //}
//            // println!("{}", self.accumulation);
//            if self.accumulation > 100000 {
//                cont = false
//            }
//        } 
//        self.accumulation 
//    }
//
//    //pub fn acc(&mut self, v: i32) -> i32 {
//    //    self.incr(1);
//    //    v 
//    //}  
//
//    pub fn incr(&mut self, v: i32) {
//        self.idx = self.idx + v;
//    }
//}
//
//#[derive(Debug)]
//struct Instruction {
//    key: String, 
//    value: i32,
//    count: i32,
//}


