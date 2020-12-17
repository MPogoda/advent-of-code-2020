use std::io::{self, BufRead};
use std::time::{Instant};

fn main() {
    let reading_timer = Instant::now();
    let input = read_input();
    println!("Read in {}ms", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    println!("Part 1 is {}, took {}ms", part1(&input), part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}ms", part2(&input), part2_timer.elapsed().as_micros());
}

fn read_input() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect()
}

fn part1(input: &[String]) -> usize {
    check_slope(input, 3, 1)
}

fn check_slope(input: &[String], x: usize, y: usize) -> usize {
    input
        .iter()
        .step_by(y)
        .enumerate()
        .filter(|(i, s)| {
            let idx = x * i % s.len();
            s.chars().nth(idx) == Some('#')
        })
        .count()
}

fn part2(input: &[String]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (x, y)| acc * check_slope(input, *x, *y))
}

