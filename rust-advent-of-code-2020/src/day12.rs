type Command = (u8, i64);

#[aoc_generator(day12)]
pub fn read_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let (cmd, val) = line.split_at(1);
            (cmd.bytes().next().unwrap(), val.parse().unwrap())
        })
        .collect()
}

type Coord = (i64, i64);
fn move_coord((y, x): &Coord, (dy, dx): &Coord, n: i64) -> Coord {
    (y + n * dy, x + n * dx)
}

fn move_direction(coord: &Coord, direction: u8, n: i64) -> Coord {
    move_coord(
        coord,
        match direction {
            b'N' => &(1, 0),
            b'E' => &(0, 1),
            b'S' => &(-1, 0),
            b'W' => &(0, -1),
            _ => panic!("Wrong direction!"),
        },
        n,
    )
}

const DIRECTIONS: &[u8; 4] = b"NESW";

fn rotate_direction(dir: usize, n: i64) -> usize {
    let rotates = (360 + n) / 90;
    dir.wrapping_add(rotates as usize) % 4
}

#[aoc(day12, part1)]
fn part1(input: &[Command]) -> i64 {
    let mut dir = 1;
    // north, east
    let mut coords = (0, 0);
    for (cmd, num) in input {
        coords = match cmd {
            b'L' => {
                dir = rotate_direction(dir, -*num);
                coords
            }
            b'R' => {
                dir = rotate_direction(dir, *num);
                coords
            }
            b'F' => move_direction(&coords, DIRECTIONS[dir], *num),
            _ => move_direction(&coords, *cmd, *num),
        }
    }
    coords.0.wrapping_abs() + coords.1.wrapping_abs()
}

fn rotate_waypoint((y, x): &Coord, n: i64) -> Coord {
    let rotates = (360 + n) / 90;
    match rotates % 4 {
        0 => (*y, *x),
        1 => (-*x, *y),
        2 => (-*y, -*x),
        3 => (*x, -*y),
        _ => panic!("Wrong rotation!"),
    }
}

#[aoc(day12, part2)]
fn part2(input: &[Command]) -> i64 {
    let mut waypoint = (1, 10);
    let mut coords = (0, 0);

    for (cmd, num) in input {
        coords = match cmd {
            b'L' => {
                waypoint = rotate_waypoint(&waypoint, -*num);
                coords
            }
            b'R' => {
                waypoint = rotate_waypoint(&waypoint, *num);
                coords
            }
            b'F' => move_coord(&coords, &waypoint, *num),
            _ => {
                waypoint = move_direction(&waypoint, *cmd, *num);
                coords
            }
        }
    }
    coords.0.wrapping_abs() + coords.1.wrapping_abs()
}
