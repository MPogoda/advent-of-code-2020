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

fn is_valid_field<'a>(field: &'a str) -> bool {
    let (key, value) = field.split_at(4);
    match key {
        "byr:" => if let Ok(v) = value.parse::<usize>() {
            return v >= 1920 && v <= 2002;
        }
        "iyr:" => if let Ok(v) = value.parse::<usize>() {
            return v >= 2010 && v <= 2020;
        }
        "eyr:" => if let Ok(v) = value.parse::<usize>() {
            return v >= 2020 && v <= 2030;
        }
        "hgt:" => {
            let num = value.trim_end_matches(|c: char| !c.is_numeric());
            let t = value.trim_start_matches(|c: char| c.is_numeric());
            match (num.parse::<usize>(), t) {
                (Ok(150..=193), "cm") => return true,
                (Ok(59..=76), "in") => return true,
                _ => return false
            }
        }
        "hcl:" => if value.len() == 7 && value.starts_with('#') {
            return value.chars().skip(1).all(|v| match v {
                '0'..='9' | 'a'..='f' => true,
                _ => false
            });
        }
        "ecl:" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
            _ => return false
        }
        "pid:" => return value.len() == 9 && value.parse::<usize>().is_ok(),
        _ => return false
    }
    false
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
        .filter(|group| {
            let iter = group.iter().flat_map(|v| v.split(' '));
            let has_cid = iter.clone().any(|x| x.starts_with("cid"));
            let count = iter.count();

            count == 7 && !has_cid || count == 8
        })
        .count()
}

fn part2(input: &[String]) -> usize {
    input
        .split(|v| v.is_empty())
        .filter(|group|
            7 == group
                .iter()
                .flat_map(|v| v.split(' '))
                .filter(|v| is_valid_field(v))
                .count()
        )
        .count()
}

