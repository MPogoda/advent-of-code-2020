use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::Hasher;

type Deck = VecDeque<u8>;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> (Deck, Deck) {
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();

    let mut switch = false;
    for line in input.lines() {
        if line.starts_with("P") {
            continue;
        }
        if line.is_empty() {
            switch = true;
            continue;
        }
        if switch {
            p2.push_front(line.parse().unwrap());
        } else {
            p1.push_front(line.parse().unwrap());
        }
    }

    (p1, p2)
}

fn play_game(p1_input: &Deck, p2_input: &Deck) -> Deck {
    let mut p1 = p1_input.clone();
    let mut p2 = p2_input.clone();
    while !p1.is_empty() && !p2.is_empty() {
        let v1 = p1.pop_back().unwrap();
        let v2 = p2.pop_back().unwrap();

        if v1 > v2 {
            p1.push_front(v1);
            p1.push_front(v2);
        } else {
            p2.push_front(v2);
            p2.push_front(v1);
        }
    }
    if p1.is_empty() {
        p2
    } else {
        p1
    }
}

#[aoc(day22, part1)]
fn part1((p1, p2): &(Deck, Deck)) -> u64 {
    play_game(p1, p2)
        .iter()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * (*v as u64))
        .sum()
}

fn calculate_hash(p1: &Deck, p2: &Deck) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(p1.as_slices().0);
    hasher.write(p1.as_slices().1);

    hasher.write_u8(100);

    hasher.write(p2.as_slices().0);
    hasher.write(p2.as_slices().1);
    hasher.finish()
}

fn take_cards(deck: &Deck, n: usize) -> Deck {
    deck.into_iter().rev().take(n).rev().cloned().collect()
}

fn play_recursive_round(p1: &mut Deck, p2: &mut Deck, memo: &mut HashSet<u64>) -> bool {
    let hash = calculate_hash(p1, p2);
    if memo.contains(&hash) {
        return true;
    }
    memo.insert(hash);

    let c1 = p1.pop_back().unwrap() as usize;
    let c2 = p2.pop_back().unwrap() as usize;

    if c1 <= p1.len() && c2 <= p2.len() {
        if play_recursive_game(&mut take_cards(p1, c1), &mut take_cards(p2, c2)).0 {
            p1.push_front(c1 as u8);
            p1.push_front(c2 as u8);
        } else {
            p2.push_front(c2 as u8);
            p2.push_front(c1 as u8);
        }
    } else {
        if c1 > c2 {
            p1.push_front(c1 as u8);
            p1.push_front(c2 as u8);
        } else {
            p2.push_front(c2 as u8);
            p2.push_front(c1 as u8);
        }
    }
    false
}

fn play_recursive_game<'a>(p1: &'a mut Deck, p2: &'a mut Deck) -> (bool, &'a mut Deck) {
    let mut memo = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        if play_recursive_round(p1, p2, &mut memo) {
            return (true, p1);
        }
    }

    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}

#[aoc(day22, part2)]
fn part2(input: &(Deck, Deck)) -> u64 {
    play_recursive_game(&mut input.0.clone(), &mut input.1.clone())
        .1
        .iter()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * (*v as u64))
        .sum()
}
