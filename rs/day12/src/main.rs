use std::io::{self, BufRead};
use std::time::{Instant};

fn main() {
    let reading_timer = Instant::now();
    let input = read_input();
    println!("Read in {}μs", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 is {}, took {}μs", part1_result, part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}μs", part2(&input, part1_result), part2_timer.elapsed().as_micros());
}

type Command = (char, i64);

fn read_input() -> Vec<Command> {
    io::stdin()
        .lock()
        .lines()
        .filter(|v| v.is_ok())
        .map(|v| {
            let s = v.unwrap();
            let (cmd, val) = s.split_at(1);
            (cmd.chars().nth(0).unwrap(), val.parse().unwrap())
        })
        .collect()
}

type Coord = (i64, i64);
fn move_coord((y, x): &Coord, (dy, dx): &Coord, n: i64) -> Coord {
    (y + n * dy, x + n * dx)
}

fn move_direction(coord: &Coord, direction: char, n: i64) -> Coord {
    move_coord(coord, match direction {
        'N' => &(1, 0),
        'E' => &(0, 1),
        'S' => &(-1, 0),
        'W' => &(0, -1),
        _ => panic!("Wrong direction!")
    }, n)
}

const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];

fn rotate_direction(dir: usize, n: i64) -> usize {
    let rotates = (360 + n) / 90;
    dir.wrapping_add(rotates as usize) % 4
}

fn part1(input: &[Command]) -> u64 {
    let mut dir = 1;
    // north, east
    let mut coords = (0, 0);
    for (cmd, num) in input {
        coords = match cmd {
            'L' => {
                dir = rotate_direction(dir, -*num);
                coords
            },
            'R' => {
                dir = rotate_direction(dir, *num);
                coords
            },
            'F' => move_direction(&coords, DIRECTIONS[dir], *num),
            _ => move_direction(&coords, *cmd, *num)
        }
    }
    (coords.0.wrapping_abs() + coords.1.wrapping_abs()) as u64
}

fn rotate_waypoint((y, x): &Coord, n: i64) -> Coord {
    let rotates = (360 + n) / 90;
    match rotates % 4 {
        0 => (*y, *x),
        1 => (-*x, *y),
        2 => (-*y, -*x),
        3 => (*x, -*y),
        _ => panic!("Wrong rotation!")
    }
}

fn part2(input: &[Command], _: u64) -> u64 {
    let mut waypoint = (1, 10);
    let mut coords = (0, 0);

    for (cmd, num) in input {
        coords = match cmd {
            'L' => {
                waypoint = rotate_waypoint(&waypoint, -*num);
                coords
            },
            'R' => {
                waypoint = rotate_waypoint(&waypoint, *num);
                coords
            },
            'F' => move_coord(&coords, &waypoint, *num),
            _ => {
                waypoint = move_direction(&waypoint, *cmd, *num);
                coords
            }
        }
    }
    (coords.0.wrapping_abs() + coords.1.wrapping_abs()) as u64
}
