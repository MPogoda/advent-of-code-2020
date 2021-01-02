use std::collections::{HashSet};
use itertools::Itertools;
use regex::Regex;

type Entry = (HashSet<String>, HashSet<String>);

lazy_static!{
    static ref RE: Regex = Regex::new(
        r"^(?P<products>.+) \(contains (?P<allergens>.+)\)$"
    ).unwrap();
}

fn parse_line(line: &str) -> Entry {
    let captures = RE.captures(line).unwrap();
    let products: HashSet<_> = captures.name("products").unwrap()
        .as_str()
        .split(' ')
        .map(|word| word.to_owned())
        .collect();

    let allergens: HashSet<_> = captures.name("allergens").unwrap()
        .as_str()
        .split(", ")
        .map(|word| word.to_owned())
        .collect();

    (products, allergens)
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(parse_line)
        .collect()
}

type Match = (String, String);

fn find_match(data: &[Entry], unmatched: &HashSet<&String>) -> Match {
    for &allergen in unmatched {
        let mut affected = data
            .iter()
            .filter(|(_, allergens)| allergens.contains(allergen))
            .map(|(products, _)| products);
        let mut common = affected.next().unwrap().clone();
        for next in affected {
            common.retain(|v| next.contains(v));
        }

        if common.len() == 1 {
            let product = common.drain().next().unwrap();
            return (product, allergen.to_owned());
        }
    }
    panic!("Cannot find the solution!");
}

fn solve(input: &[Entry]) -> (Vec<Entry>, Vec<(String, String)>) {
    let mut unmatched: HashSet<_> = input
        .iter()
        .flat_map(|(_, allergens)| allergens.iter())
        .collect();
    let mut data = input.to_vec();

    let mut matches = Vec::new();

    while !unmatched.is_empty() {
        let (product, allergen) = find_match(&data, &unmatched);
        for (products, allergens) in &mut data {
            products.retain(|v| *v != product);
            allergens.retain(|v| *v != allergen);
        }
        unmatched.remove(&allergen);
        matches.push((product, allergen));
    }

    (data, matches)
}

#[aoc(day21, part1)]
fn part1(input: &[Entry]) -> usize {
    solve(input).0.iter().map(|(v, _)| v.len()).sum()
}

#[aoc(day21, part2)]
fn part2(input: &[Entry]) -> String {
    let (_, mut matches) = solve(input);
    matches.sort_by_cached_key(|(_, allergen)| allergen.clone());
    matches.drain(0..).map(|(product, _)| product)
        .collect_vec()
        .join(",")
}
