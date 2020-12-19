use std::iter::FromIterator;
use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::{HashMap, HashSet};

fn main() {
    let reading_timer = Instant::now();
    let input = read_input();
    println!("Read in {}μs", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    let part1_result = part1(&input);
    println!("Part 1 is {}, took {}μs", part1_result, part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}μs", part2(&input, part1_result), part2_timer.elapsed().as_micros());
}

fn read_input() -> Vec<u8> {
    let mut result: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap().parse().unwrap())
        .collect();

    result.sort_unstable();
    result
}

fn part1(input: &[u8]) -> u64 {
    let (j1, j3) = input
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .fold((1, 1), |(j1, j3), x| match x {
            1 => (j1 + 1, j3),
            3 => (j1, j3 + 1),
            _ => (j1, j3)
        });

    j1 * j3
}

fn recurse(values: &HashSet<&u8>, visited: &mut HashMap<u8, u64>, value: u8, target: u8) -> u64 {
    if let Some(&v) = visited.get(&value) { return v }
    if value == target { return 1 }

    let r = [1u8,2,3]
        .iter()
        .filter(|v| values.contains(&(value + *v)))
        .fold(0, |acc, v| acc + recurse(values, visited, value + *v, target));
    visited.insert(value, r);
    r
}

fn part2(input: &[u8], _: u64) -> u64 {
    recurse(
        &HashSet::from_iter(input.iter()),
        &mut HashMap::new(),
        0,
        *input.last().unwrap()
    )
}
