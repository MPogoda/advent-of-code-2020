use std::convert::TryFrom;
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
    input
        .split(|v| v.is_empty())
        .map(|group| {
            let mut seen = [false; 26];

            group.iter().map(|v| v.chars()).flatten()
                .for_each(|ch| {
                    let v = ch.to_digit(36).unwrap() - 10;
                    seen[usize::try_from(v).unwrap()] = true;
                });

            seen.iter().filter(|v| **v).count()
        })
        .fold(0, |acc, v| acc + v)
}

fn part2(input: &[String]) -> usize {
    input
        .split(|v| v.is_empty())
        .map(|group| ('a'..='z')
            .filter(|ch| group.iter().all(|s| s.find(*ch).is_some()))
            .count()
        ).fold(0, |acc, v| acc + v)
}

