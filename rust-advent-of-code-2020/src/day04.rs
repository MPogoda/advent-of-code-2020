use itertools::Itertools;

fn is_valid_field(field: &str) -> bool {
    if field.is_empty() { return false }
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

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let groups = input
        .lines()
        .group_by(|v| v.is_empty());

    groups
        .into_iter()
        .map(|(_, group)| {
            group
                .into_iter()
                .flat_map(|v| v.split(' '))
                .filter(|v| !v.starts_with("cid"))
                .count()
        })
        .filter(|&v| v == 7)
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let groups = input
        .lines()
        .group_by(|v| v.is_empty());

    groups
        .into_iter()
        .map(|(_, group)| {
            group
                .into_iter()
                .flat_map(|v| v.split(' '))
                .filter(|v| is_valid_field(v))
                .count()
        })
        .filter(|&v| v == 7)
        .count()
}
