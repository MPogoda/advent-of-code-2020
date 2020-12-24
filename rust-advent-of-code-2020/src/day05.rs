use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc_generator(day5)]
pub fn read_input(input: &[u8]) -> Vec<u16> {
    input
        .split(|&ch| ch == b'\n')
        .map(|v| {
            let (row_s, seat_s) = v.split_at(7);
            let row = u16::from_str_radix(
                &row_s.iter().map(|&x| if x == b'F' { '0' } else { '1'}).collect::<String>(),
                2
            ).unwrap();
            let seat = u16::from_str_radix(
                &seat_s.iter().map(|&x| if x == b'L' { '0' } else { '1' }).collect::<String>(),
                2
            ).unwrap();
            calculate_id(row, seat)
        })
        .collect()
}

fn calculate_id(row: u16, seat: u16) -> u16 {
    row * 8 + seat
}

#[aoc(day5, part1)]
pub fn part1(input: &[u16]) -> u16 {
    *input
        .iter()
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[u16]) -> u16 {
    let places = HashSet::<&u16>::from_iter(input);
    let mut seen_place = false;
    for row in 0..=127 {
        for seat in 0..=7 {
            let id = calculate_id(row, seat);
            if places.contains(&id) {
                seen_place = true;
            } else if seen_place {
                return id
            }
        }
    }
    1
}
