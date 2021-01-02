use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use itertools::Itertools;
use regex::bytes::{Captures, Regex};

type Ticket = Vec<usize>;
type Rules = Vec<Vec<u64>>;
type Input = (Rules, Vec<u64>, Vec<u64>, Ticket, Vec<Ticket>);

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<name>.+): (?P<n1>\d+)-(?P<n2>\d+) or (?P<n3>\d+)-(?P<n4>\d+)$").unwrap();
}

fn hash(name: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(name);
    hasher.finish()
}

fn parse<'t>(captures: &Captures<'t>, key: &str) -> usize {
    std::str::from_utf8(captures.name(key).unwrap().as_bytes())
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(|ch| ch == ',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Input {
    let groups = input.lines().group_by(|line| line.is_empty());
    let mut group_iter = groups.into_iter();

    let mut rules = Vec::with_capacity(1_000);
    rules.resize(1_000, Vec::with_capacity(20));
    let mut interesting_rules = Vec::with_capacity(6);
    let mut all_rules = Vec::with_capacity(20);

    for line in group_iter.next().unwrap().1 {
        let captures = RE.captures(line.as_bytes()).unwrap();
        let name = captures.name("name").unwrap().as_bytes();
        let name_hash = hash(name);
        all_rules.push(name_hash);
        if name.starts_with(b"departure") {
            interesting_rules.push(name_hash)
        }
        for i in parse(&captures, "n1")..=parse(&captures, "n2") {
            rules[i].push(name_hash);
        }
        for i in parse(&captures, "n3")..=parse(&captures, "n4") {
            rules[i].push(name_hash);
        }
    }

    group_iter.next();
    let this_ticket = parse_ticket(group_iter.next().unwrap().1.skip(1).next().unwrap());

    group_iter.next();
    let nearby_tickets = group_iter
        .next()
        .unwrap()
        .1
        .skip(1)
        .map(parse_ticket)
        .collect();

    (
        rules,
        all_rules,
        interesting_rules,
        this_ticket,
        nearby_tickets,
    )
}

#[aoc(day16, part1)]
fn part1((rules, _, _, _, nearby_tickets): &Input) -> usize {
    nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter().filter(|num| rules[**num].is_empty()))
        .sum()
}

fn clean_non_ambiguous(possible_names: &mut Vec<Vec<u64>>, name: u64) {
    let mut next = Vec::new();
    for this_names in possible_names.iter_mut() {
        if this_names.len() <= 1 {
            continue;
        }
        this_names.retain(|&n| n != name);
        if this_names.len() == 1 {
            next.push(this_names[0]);
        }
    }

    for n in next {
        clean_non_ambiguous(possible_names, n);
    }
}

#[aoc(day16, part2)]
fn part2((rules, all_rules, interesting_rules, this_ticket, nearby_tickets): &Input) -> usize {
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|&num| !rules[num].is_empty()));

    let mut possible_names = Vec::with_capacity(this_ticket.len());
    for _ in 0..this_ticket.len() {
        possible_names.push(all_rules.clone());
    }

    for ticket in valid_tickets {
        for (i, num) in ticket.iter().enumerate() {
            if possible_names[i].len() <= 1 {
                continue;
            }
            possible_names[i].retain(|name| rules[*num].contains(name));
            if possible_names[i].len() == 1 {
                let name = possible_names[i][0];
                clean_non_ambiguous(&mut possible_names, name);
            }
        }
    }
    this_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| interesting_rules.contains(&possible_names[*i][0]))
        .map(|(_, v)| v)
        .product()
}
