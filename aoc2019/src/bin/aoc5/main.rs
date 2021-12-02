pub fn execute_at(xs: &Vec<i32>, ind: usize, input: &i32) -> Result<(Vec<i32>, usize), Vec<i32>> {
    let mut ys = xs.clone();

    let op = xs[ind] % 100;
    let pmode = |i: usize| ((xs[ind] / (10i32.pow(i as u32 + 1))) % 10) == 0;
    let fetch = |i: usize| {
        if pmode(i) {
            xs[xs[ind+i] as usize]
        } else {
            xs[ind+i]
        }
    };
    let mut store = |i: usize, x: i32| ys[xs[ind+i] as usize] = x;

    match op {
        // Exit
        99 => Err(ys),

        // Add
        1 => {
            store(3, fetch(1) + fetch(2));
            Ok((ys, ind+4))
        },

        // Multiply
        2 => {
            store(3, fetch(1) * fetch(2));
            Ok((ys, ind+4)) 
        },

        // Read input
        3 => {
            store(1, *input);
            Ok((ys, ind+2))
        },

        // Output
        4 => {
            print_output(&fetch(1));
            Ok((ys, ind+2))
        },

        // Non-zero
        5 => {
            if fetch(1) != 0 {
                Ok((ys, fetch(2) as usize))
            } else {
                Ok((ys, ind+3))
            }
        },

        // Is zero
        6 => {
            if fetch(1) == 0 {
                Ok((ys, fetch(2) as usize))
            } else {
                Ok((ys, ind+3))
            }
        },

        // Less than
        7 => {
            store(3, {
                if fetch(1) < fetch(2) { 1 }
                else { 0 }
            });
            Ok((ys, ind+4)) 
        },

        // Equals
        8 => {
            store(3, {
                if fetch(1) == fetch(2) { 1 }
                else { 0 }
            });
            Ok((ys, ind+4)) 
        },

        // Error
        x => {
            println!("Invalid operation {} at {}!", x, ind);
            Err(ys)
        },
    }
}

pub fn execute_program(xs: &Vec<i32>, input: &i32) -> Vec<i32> {
    let mut ip = 0usize;
    let mut ys = xs.clone();
    loop {
        match execute_at(&ys, ip, input) {
            Ok((x, new_ip)) => {
                ys = x;
                ip = new_ip;
            },
            Err(x) => {
                return x; 
            },
        }
    }
}

fn print_output(x: &i32) {
    if *x != 0 {
        println!("{}", x);
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Error opening file!");

    let xs: Vec<i32> = contents
        .lines().next().unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    print!("Part 1: ");
    execute_program(&xs, &1);

    print!("Part 2: ");
    execute_program(&xs, &5);
}
