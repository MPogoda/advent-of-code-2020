use std::collections::HashMap;
use regex::Regex;

enum Input {
    Mask(Vec<u8>),
    Mem(u64, u64)
}
lazy_static! {
    static ref RE: Regex = Regex::new(
        r"^(mask = (?P<mask>.+)|mem\[(?P<mem>\d+)\] = (?P<val>\d+))$"
    ).unwrap();
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let captures = RE.captures(line).unwrap();
            if let Some(mask) = captures.name("mask") {
                Input::Mask(
                    mask.as_str().as_bytes().to_vec()
                )
            } else {
                Input::Mem(
                    captures.name("mem").unwrap().as_str()
                        .parse().unwrap(),
                    captures.name("val").unwrap().as_str()
                        .parse().unwrap()
                )
            }
        })
        .collect()
}

// treat mask as two parts:
//  - .0  will be AND
//  - .1 will be OR
fn convert_mask(mask: &[u8]) -> (u64, u64) {
    let mut mask_0 = u64::MAX;
    let mut mask_1 = 0;
    let mut power = 1;
    for v in mask.iter().rev() {
        match v {
            b'1' => { mask_1 += power },
            b'0' => { mask_0 -= power },
            _ => {}
        }

        power *= 2;
    }
    (mask_0, mask_1)
}

#[aoc(day14, part1)]
fn part1(input: &[Input]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = (u64::MAX, 0);
    for line in input {
        match line {
            Input::Mask(new_mask) => { mask = convert_mask(new_mask); },
            Input::Mem(addr, val) => { mem.insert(addr, val & mask.0 | mask.1); }
        }
    }
    mem.values().sum()
}

fn recurse(floating: &[u64], base: (u64, u64), result: &mut Vec<(u64, u64)>) {
    if floating.is_empty() { return result.push(base) }
    let h = floating[0];
    recurse(&floating[1..], (base.0 - h, base.1), result);
    recurse(&floating[1..], (base.0, base.1 + h), result);
}

fn extract_masks(mask_input: &[u8]) -> Vec<(u64, u64)> {
    let mut mask = (u64::MAX, 0);
    let mut floating = Vec::new();
    let mut power = 1;
    for v in mask_input.iter().rev() {
        match v {
            b'1' => { mask.1 += power },
            b'X' => { floating.push(power) },
            _ => {}
        }
        power *= 2;
    }
    let mut result = Vec::new();
    recurse(&floating, mask, &mut result);

    result
}

#[aoc(day14, part2)]
fn part2(input: &[Input]) -> u64 {
    let mut mem = HashMap::new();
    let mut masks = Vec::new();
    for line in input {
        match line {
            Input::Mask(new_mask) => { masks = extract_masks(new_mask) },
            Input::Mem(addr, val) => {
                for mask in &masks {
                    mem.insert(addr & mask.0 | mask.1, *val);
                }
            }
        }
    }
    mem.values().sum()
}
