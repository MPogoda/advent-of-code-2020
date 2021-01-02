use itertools::Itertools;
use std::str;

fn is_valid_field(field: &[u8]) -> bool {
    if field.is_empty() {
        return false;
    }
    let (key, value) = field.split_at(4);
    match str::from_utf8(key).unwrap() {
        "byr:" => {
            if let Ok(v) = str::from_utf8(value).unwrap().parse::<u16>() {
                return v >= 1920 && v <= 2002;
            }
        }
        "iyr:" => {
            if let Ok(v) = str::from_utf8(value).unwrap().parse::<u16>() {
                return v >= 2010 && v <= 2020;
            }
        }
        "eyr:" => {
            if let Ok(v) = str::from_utf8(value).unwrap().parse::<u16>() {
                return v >= 2020 && v <= 2030;
            }
        }
        "hgt:" => {
            let s = str::from_utf8(value).unwrap();
            let num = s.trim_end_matches(|c: char| !c.is_numeric());
            let t = s.trim_start_matches(|c: char| c.is_numeric());
            match (num.parse::<u16>(), t) {
                (Ok(150..=193), "cm") => return true,
                (Ok(59..=76), "in") => return true,
                _ => return false,
            }
        }
        "hcl:" => {
            if value.len() == 7 && value[0] == b'#' {
                return value.iter().skip(1).all(|v| match v {
                    b'0'..=b'9' | b'a'..=b'f' => true,
                    _ => false,
                });
            }
        }
        "ecl:" => match str::from_utf8(value).unwrap() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
            _ => return false,
        },
        "pid:" => return value.len() == 9 && value.iter().all(|&v| v >= b'0' && v <= b'9'),
        _ => return false,
    }
    false
}

#[aoc(day4, part1)]
pub fn part1(input: &[u8]) -> usize {
    let groups = input.split(|&ch| ch == b'\n').group_by(|v| v.is_empty());

    groups
        .into_iter()
        .map(|(_, group)| {
            group
                .into_iter()
                .flat_map(|v| v.split(|&ch| ch == b' '))
                .filter(|v| !v.starts_with(&[b'c', b'i', b'd']))
                .count()
        })
        .filter(|&v| v == 7)
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[u8]) -> usize {
    let groups = input.split(|&ch| ch == b'\n').group_by(|v| v.is_empty());

    groups
        .into_iter()
        .map(|(_, group)| {
            group
                .into_iter()
                .flat_map(|v| v.split(|&ch| ch == b' '))
                .filter(|v| is_valid_field(v))
                .count()
        })
        .filter(|&v| v == 7)
        .count()
}
