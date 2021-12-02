use std::fs;

//fn get_values(d: &Vec<u32>, pc: usize) -> (u32, u32) {
//    let (op1, op2) = (d[pc + 1], d[pc + 2]);
//    (d[op1 as usize], d[op2 as usize])
//}

//fn execute_program(mut data: Vec<u32>) -> Vec<u32> {
//    let mut pc: usize = 0;
//
//    loop {
//        let opcode = data[pc];
//
//        match opcode {
//            // sum
//            1 => {
//                let (v1, v2) = get_values(&data, pc);
//                let target_location = data[pc + 3];
//                data[target_location as usize] = v1 + v2;
//            },
//            // mul
//            2 => {
//                let (v1, v2) = get_values(&data, pc);
//                let target_location = data[pc + 3];
//                data[target_location as usize] = v1 * v2;
//            },
//            99 => { break; },
//            _ => { },
//        }
//
//        pc += 4;
//    }
//
//    data
//}

fn run_program(initial_program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut program : Vec<i32> = initial_program.clone();
    program[1] = noun;
    program[2] = verb;

    let mut counter = 0;
    loop {
        match program[counter] {
            1 => {
                let a_index = program[counter + 1];
                let b_index = program[counter + 2];
                let a = program[a_index as usize];
                let b = program[b_index as usize];

                let output_index = program[counter + 3];
                let output = a + b;
                program[output_index as usize] = output;
                counter = counter + 4;
            },
            2 => {
                let a_index = program[counter + 1];
                let b_index = program[counter + 2];
                let a = program[a_index as usize];
                let b = program[b_index as usize];

                let output_index = program[counter + 3];
                let output = a * b;
                program[output_index as usize] = output;
                counter = counter + 4;
            },
            99 => {
                // println!("Program halted at 99");
                break;
            },
            _ => {
                panic!("Program halted at unexpected input");
            }
        }
    }
    program[0]
}

fn search(data: &Vec<i32>, search: i32) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut d2: Vec<i32> = data.clone();
            d2[1] = noun;
            d2[2] = verb;
            let result = run_program(&d2, noun, verb);
            if result == search {
                return 100 * noun + verb;
            }
        }
    }
    0
}


fn main() {
    let f = fs::read_to_string("input.txt").expect("File doesn't exists"); 
     
    let data: Vec<i32> = f
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part1: position 0 is [ {} ]", run_program(&data.clone(),12,2));
    println!("part2: {}", search(&data.clone(), 19690720))
}


