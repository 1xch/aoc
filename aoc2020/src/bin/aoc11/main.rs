use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
enum TileKind {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl TileKind {
    fn flip(&mut self) {
        *self = match self {
            TileKind::Floor => TileKind::Floor,
            TileKind::EmptySeat => TileKind::OccupiedSeat,
            TileKind::OccupiedSeat => TileKind::EmptySeat,
        }
    }
}

impl From<char> for TileKind {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::EmptySeat,
            '#' => Self::OccupiedSeat,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position {
    column: i32,
    row: i32,
}

impl Position {
    fn adjacent_position(&self, direction: &Direction) -> Self {
        let (column_delta, row_delta) = direction.into();

        Position {
            column: self.column + column_delta,
            row: self.row + row_delta,
        }
    }

    fn adjacent_positions(&self) -> Vec<Position> {
        use Direction::*;

        let mut result = Vec::with_capacity(8);

        for direction in &[
            Northwest, North, Northeast, West, East, Southwest, South, Southeast,
        ] {
            result.push(self.adjacent_position(direction));
        }

        result
    }
}

#[derive(Clone)]
struct Layout(HashMap<Position, TileKind>);

#[derive(Debug)]
enum Direction {
    Northwest,
    North,
    Northeast,
    West,
    East,
    Southwest,
    South,
    Southeast,
}

impl From<&Direction> for (i32, i32) {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Northwest => (-1, -1),
            Direction::North => (0, -1),
            Direction::Northeast => (1, -1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::Southwest => (-1, 1),
            Direction::South => (0, 1),
            Direction::Southeast => (1, 1),
        }
    }
}

// #[aoc_generator(day11)]
fn parse_input(input: &str) -> Layout {
    let mut tiles = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            tiles.insert(
                Position {
                    column: column as i32,
                    row: row as i32,
                },
                c.into(),
            );
        }
    }

    Layout(tiles)
}

#[derive(Eq, PartialEq)]
enum RoundResult {
    Changed,
    Stabilized,
}

fn count_adjacent_occupied(layout: &Layout, position: &Position) -> usize {
    let mut count = 0;

    for adjacent_position in position.adjacent_positions() {
        match layout.0.get(&adjacent_position) {
            Some(adjacent_tile_kind) if *adjacent_tile_kind == TileKind::OccupiedSeat => count += 1,
            _ => {}
        }
    }

    count
}

fn count_visible_occupied(layout: &Layout, position: &Position) -> usize {
    use Direction::*;

    let mut count = 0;

    for direction in &[
        Northwest, North, Northeast, West, East, Southwest, South, Southeast,
    ] {
        let mut next_position = position.adjacent_position(&direction);

        while let Some(tile_kind) = layout.0.get(&next_position) {
            match tile_kind {
                TileKind::OccupiedSeat => {
                    count += 1;
                    break;
                }
                TileKind::EmptySeat => break,
                TileKind::Floor => next_position = next_position.adjacent_position(&direction),
            }
        }
    }

    count
}

fn empty_seat_flip_rule(
    layout: &Layout,
    position: &Position,
    count_function: fn(&Layout, &Position) -> usize,
) -> bool {
    count_function(layout, position) == 0
}

fn occupied_seat_flip_rule(
    layout: &Layout,
    position: &Position,
    count_function: fn(&Layout, &Position) -> usize,
    threshold: usize,
) -> bool {
    count_function(layout, position) >= threshold
}

fn round(
    layout: &mut Layout,
    count_function: fn(&Layout, &Position) -> usize,
    occupied_seats_threshold: usize,
) -> RoundResult {
    let mut to_flip = Vec::new();

    for (position, tile_kind) in layout.0.iter() {
        if (*tile_kind == TileKind::EmptySeat
            && empty_seat_flip_rule(&layout, &position, count_function))
            || (*tile_kind == TileKind::OccupiedSeat
                && occupied_seat_flip_rule(
                    &layout,
                    &position,
                    count_function,
                    occupied_seats_threshold,
                ))
        {
            to_flip.push(position.clone())
        }
    }

    for position in to_flip.iter() {
        if let Some(tile_kind) = layout.0.get_mut(position) {
            (*tile_kind).flip()
        }
    }

    use RoundResult::*;

    if !to_flip.is_empty() {
        Changed
    } else {
        Stabilized
    }
}

fn stabilize(
    layout: &mut Layout,
    count_function: fn(&Layout, &Position) -> usize,
    occupied_seats_threshold: usize,
) -> usize {
    use RoundResult::*;
    let mut round_result = Changed;

    while round_result == Changed {
        round_result = round(layout, count_function, occupied_seats_threshold);
    }

    layout
        .0
        .values()
        .filter(|&tile_kind| *tile_kind == TileKind::OccupiedSeat)
        .count()
}

// #[aoc(day11, part1)]
fn part1(layout: &Layout) -> usize {
    let mut layout = layout.clone();
    stabilize(&mut layout, count_adjacent_occupied, 4)
}

// #[aoc(day11, part2)]
fn part2(layout: &Layout) -> usize {
    let mut layout = layout.clone();
    stabilize(&mut layout, count_visible_occupied, 5)
}

fn main() {
    let layout = parse_input(include_str!("input.txt")); 
    println!("Answer to part 1: {}", part1(&layout));
    println!("Answer to part 2: {}", part2(&layout));
}


// use std::collections::HashMap;
//use std::fmt;
//
//fn main() {
//    println!("Answer to part 1: {}", part1(include_str!("input.txt")));
//    println!("Answer to part 2: {}", part2(include_str!("input.txt")));
//}
//
//#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
//enum Square {
//    Floor = 0,
//    Chair = 1,
//    Person = 2,
//}
//
//impl Default for Square {
//    fn default() -> Self {
//        Self::Floor
//    }
//}
//
//impl fmt::Display for Square {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            Self::Floor  => write!(f, "."),
//            Self::Chair  => write!(f, "L"),
//            Self::Person => write!(f, "#"),
//        }
//    }
//}
//
//struct Grid {
//    x: usize,
//    y: usize,
//    data: Vec<Vec<Square>>,
//}
//
//fn parse_grid(inp: &str) -> Grid {
//    let input: Vec<&str> = inp.lines().collect(); 
//    let ydim: usize = input.len();
//    let xdim: usize = input.iter().next().unwrap().len();
//    let mut chairs = vec![vec![Square::Floor; ydim]; xdim];
//    for y in 0..input.len() as usize {
//        let mut line = input[y].chars();
//        for x in 0..xdim as usize {
//            match line.next().unwrap().to_string().as_ref() {
//                "L" => chairs[x][y] = Square::Chair,
//                "." => chairs[x][y] = Square::Floor,
//                "#" => unreachable!("Found a person in input file."),
//                other => panic!("Unknown map square: {}", other),
//            }
//        }
//    }
//    Grid {
//        x: xdim,
//        y: ydim,
//        data: chairs,
//    }
//    //let mut g:Grid = Vec::new();
//    //inp.lines()
//    //    .enumerate()
//    //    .for_each(|(idx, l)| {
//    //        let d:Vec<Square> = l.chars().map(|c| { 
//    //            match c {
//    //                'L' => Square::Chair,
//    //                // '#' => Status::Occupied,
//    //                _ => Square::Floor,
//    //                }
//    //       }).collect();
//    //        g.insert(idx,d);
//    //    });
//    //g
//}
//
//static ALL_DIRS: &'static [(i64,i64)] = &[(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
//
//fn count_neighbors_p1(map: &Vec<Vec<Square>>, x: i64, y: i64, xdim: i64, ydim: i64) -> i64 {
//    let mut neighbors = 0;
//    for dir in ALL_DIRS {
//        let (nx,ny) = (x + dir.0, y + dir.1);
//        if (0..xdim).contains(&nx) && (0..ydim).contains(&ny) {
//            match map[nx as usize][ny as usize] {
//                Square::Person => { neighbors += 1; continue; },
//                _ => {},
//            }
//        } else {
//            continue;
//        }
//    }
//    neighbors
//}
//
//fn count_neighbors_p2(map: &Vec<Vec<Square>>, x: i64, y: i64, xdim: i64, ydim: i64) -> i64 {
//    let mut neighbors = 0;
//    'outer: for dir in ALL_DIRS {
//        for i in 1.. {
//            let (nx,ny) = (x + i*dir.0, y + i*dir.1);
//            if (0..xdim).contains(&nx) && (0..ydim).contains(&ny) {
//                match map[nx as usize][ny as usize] {
//                    Square::Person => { neighbors += 1; continue 'outer; },
//                    Square::Chair => continue 'outer,
//                    _ => {},
//                }
//            } else {
//                continue 'outer;
//            }
//        }
//    }
//    neighbors
//}
//
//fn seat_map(input: &Vec<&str>) -> Vec<Vec<Square>> {
//    let ydim: usize = input.len();
//    let xdim: usize = input.iter().next().unwrap().len();
//    let mut chairs = vec![vec![Square::Floor; ydim]; xdim];
//    for y in 0..input.len() as usize {
//        let mut line = input[y].chars();
//        for x in 0..xdim as usize {
//            match line.next().unwrap().to_string().as_ref() {
//                "L" => chairs[x][y] = Square::Chair,
//                "." => chairs[x][y] = Square::Floor,
//                "#" => unreachable!("Found a person in input file."),
//                other => panic!("Unknown map square: {}", other),
//            }
//        }
//    }
//    chairs
//}
//
//fn part1(inp:&str) -> usize {
//    // let ydim: usize = inp.len();
//    // let xdim: usize = inp.iter().next().unwrap().len();
//    let mut chairs: Grid = parse_grid(inp);
//    'main_lp: loop {
//        let current = chairs.data.clone();
//        let mut changes = 0;
//        for y in 0..chairs.y as usize {
//            for x in 0..chairs.y as usize {
//
//                // Count neighbors
//                let people = count_neighbors_p1(&current, x as i64, y as i64, chairs.x as i64, chairs.y as i64);
//
//                // If a seat is empty (L) and there are no occupied seats
//                // adjacent to it, the seat becomes occupied.
//                if current[x][y] == Square::Chair && people == 0 {
//                    chairs.data[x][y] = Square::Person;
//                    changes += 1;
//                }
//
//                // If a seat is occupied (#) and four or more seats adjacent to it are
//                // also occupied, the seat becomes empty.
//                if current[x][y] == Square::Person && people >= 4 {
//                    chairs.data[x][y] = Square::Chair;
//                    changes += 1;
//                }
//            }
//        }
//        // Check if previously seen
//        /*match seen.get(&chairs) {
//            Some(_) => { break 'main_lp; },
//            None => { seen.insert(chairs.clone()); },
//        }*/
//        if changes == 0 { break 'main_lp; }
//    }
//
//    chairs.data
//        .iter()
//        .flat_map(|x| x.iter().filter(|x| x == &&Square::Person))
//        .count()
//    //println!("{:?}", g);
//    // 0
//}
//
//fn part2(_inp:&str) -> usize {
//    0
//}
