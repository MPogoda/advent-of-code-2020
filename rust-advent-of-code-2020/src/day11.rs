use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
}
pub type Field = Vec<Vec<Seat>>;
type Coord<T = usize> = (T, T);

#[aoc_generator(day11)]
pub fn read_input(input: &str) -> Field {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|ch| if ch == b'L' { Seat::Empty } else { Seat::Floor })
                .collect()
        })
        .collect()
}

fn create_iter(v0: usize, max_v: usize) -> std::ops::Range<usize> {
    std::ops::Range {
        start: v0.checked_sub(1).unwrap_or(0),
        end: max_v.min(v0 + 2),
    }
}

fn simple_neighbours(field: &Field, (x0, y0): Coord) -> usize {
    create_iter(y0, field.len())
        .cartesian_product(create_iter(x0, field[0].len()))
        .filter(|&(y, x)| {
            if x == x0 && y == y0 {
                false
            } else {
                field[y][x] == Seat::Occupied
            }
        })
        .count()
}

fn evolve(
    field: Field,
    neighbours: fn(&Field, Coord) -> usize,
    occupied_tolerance: usize,
) -> (bool, u16, Field) {
    let mut changed = false;
    let mut occupied = 0;

    let next = field
        .iter()
        .enumerate()
        .map(|(j, row)| {
            row.iter()
                .enumerate()
                .map(|(i, v)| {
                    if *v == Seat::Floor {
                        return *v;
                    }
                    let n = neighbours(&field, (i, j));
                    match (v, n) {
                        (Seat::Empty, 0) => {
                            occupied += 1;
                            changed = true;
                            Seat::Occupied
                        }
                        (Seat::Occupied, _) => {
                            if n >= occupied_tolerance {
                                changed = true;
                                Seat::Empty
                            } else {
                                occupied += 1;
                                *v
                            }
                        }
                        _ => *v,
                    }
                })
                .collect()
        })
        .collect();

    (changed, occupied, next)
}

#[aoc(day11, part1)]
fn part1(input: &Field) -> u16 {
    let mut prev = input.to_vec();
    loop {
        let (changed, occupied, next) = evolve(prev, simple_neighbours, 4);
        if !changed {
            return occupied;
        }
        prev = next;
    }
}

fn go_in_direction(field: &Field, (x0, y0): Coord, (dy, dx): Coord<isize>) -> bool {
    if dx == 0 && dy == 0 {
        return false;
    }
    let mut x = x0.wrapping_add(dx as usize);
    let mut y = y0.wrapping_add(dy as usize);
    while x < field[0].len() && y < field.len() {
        match field[y][x] {
            Seat::Occupied => return true,
            Seat::Empty => return false,
            Seat::Floor => {
                x = x.wrapping_add(dx as usize);
                y = y.wrapping_add(dy as usize);
            }
        }
    }
    false
}

fn far_neighbours(field: &Field, coord: Coord) -> usize {
    (-1isize..=1)
        .cartesian_product(-1isize..=1)
        .filter(|&dyx| go_in_direction(field, coord, dyx))
        .count()
}

#[aoc(day11, part2)]
fn part2(input: &Field) -> u16 {
    let mut prev = input.to_vec();
    loop {
        let (changed, occupied, next) = evolve(prev, far_neighbours, 5);
        if !changed {
            return occupied;
        }
        prev = next;
    }
}
