use std::io::{self, BufRead};
use std::time::{Instant};

fn main() {
    let reading_timer = Instant::now();
    let mut input = read_input();
    println!("Read in {}ms", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    println!("Part 1 is {}, took {}ms", part1(&input), part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    input.sort();
    println!("Part 2 is {}, took {}ms", part2(&input), part2_timer.elapsed().as_micros());
}

fn read_input() -> Vec<u16> {
    io::stdin()
        .lock()
        .lines()
        .filter(|v| v.is_ok())
        .map(|v| {
            let string = v.unwrap();
            let (row_s, seat_s) = string.as_str().split_at(7);
            let row = u16::from_str_radix(&row_s.chars().map(|x| if x == 'F' { '0' } else { '1'}).collect::<String>(), 2).unwrap();
            let seat = u16::from_str_radix(&seat_s.chars().map(|x| if x == 'L' { '0' } else { '1'}).collect::<String>(), 2).unwrap();
            calculate_id(row, seat)
        })
        .collect()
}

fn calculate_id(row: u16, seat: u16) -> u16 {
    row * 8 + seat
}

fn part1(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

fn part2(input: &[u16]) -> u16 {
    let mut seen_place = false;
    for row in 0..=127 {
        for seat in 0..=7 {
            let id = calculate_id(row, seat);
            if input.binary_search(&id).is_ok() {
                seen_place = true;
            } else if seen_place {
                return id;
            }
        }
    }
    1
}

