use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<u8> {
    let mut result: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    result.sort_unstable();
    result
}

#[aoc(day10, part1)]
pub fn part1(input: &[u8]) -> u64 {
    let (j1, j3) = input
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .fold((1, 1), |(j1, j3), x| match x {
            1 => (j1 + 1, j3),
            3 => (j1, j3 + 1),
            _ => (j1, j3),
        });

    j1 * j3
}

fn recurse(values: &HashSet<&u8>, visited: &mut HashMap<u8, u64>, value: u8, target: u8) -> u64 {
    if let Some(&v) = visited.get(&value) {
        return v;
    }
    if value == target {
        return 1;
    }

    let r = [1u8, 2, 3]
        .iter()
        .filter(|v| values.contains(&(value + *v)))
        .map(|v| recurse(values, visited, value + *v, target))
        .fold1(|lhs, rhs| lhs + rhs)
        .unwrap();
    visited.insert(value, r);
    r
}

#[aoc(day10, part2)]
fn part2(input: &[u8]) -> u64 {
    recurse(
        &HashSet::from_iter(input.iter()),
        &mut HashMap::new(),
        0,
        *input.last().unwrap(),
    )
}
