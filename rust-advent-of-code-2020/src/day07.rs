use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hasher;
use std::iter::FromIterator;

type Dep = (u64, usize);
type Input = HashMap<u64, Vec<Dep>>;

fn hash(w1: &[u8], w2: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(w1);
    hasher.write(w2);
    hasher.finish()
}

#[aoc_generator(day7)]
pub fn parse_input(input: &[u8]) -> Input {
    HashMap::from_iter(input.split(|&ch| ch == b'\n').map(|line| {
        let mut words = line.split(|&ch| ch == b' ').peekable();
        let n1 = words.next().unwrap();
        let n2 = words.next().unwrap();
        let name = hash(n1, n2);

        words.next();
        words.next();

        let mut deps = Vec::new();
        while words.peek().is_some() {
            let w1 = words.next().unwrap();
            if w1.starts_with(b"no") {
                return (name, deps);
            }
            let w2 = words.next().unwrap();
            let w3 = words.next().unwrap();
            deps.push((
                hash(w2, w3),
                std::str::from_utf8(w1).unwrap().parse().unwrap(),
            ));
            words.next();
        }
        (name, deps)
    }))
}

const DESIRED_BAG: [&[u8]; 2] = [b"shiny", b"gold"];

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    let mut deps = HashMap::new();

    input
        .iter()
        .flat_map(|(outer, deps)| deps.iter().map(move |(dep, _)| (dep, outer)))
        .for_each(|(inner, outer)| {
            if !deps.contains_key(inner) {
                deps.insert(*inner, Vec::new());
            }
            deps.get_mut(inner).unwrap().push(*outer);
        });

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(hash(DESIRED_BAG[0], DESIRED_BAG[1]));
    while !queue.is_empty() {
        let this_bag = queue.pop_front().unwrap();
        visited.insert(this_bag);
        deps.get(&this_bag)
            .iter()
            .flat_map(|vs| vs.iter())
            .for_each(|dep| {
                if !visited.contains(dep) {
                    queue.push_back(*dep);
                }
            });
    }
    visited.len() - 1
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    let mut stack = Vec::new();
    let mut count = 0usize;
    stack.push((1usize, hash(DESIRED_BAG[0], DESIRED_BAG[1])));
    while !stack.is_empty() {
        let (this_bag_count, this_bag_name) = stack.pop().unwrap();
        count += this_bag_count;
        input
            .get(&this_bag_name)
            .unwrap()
            .iter()
            .for_each(|(inner_bag_name, inner_bag_count)| {
                stack.push((this_bag_count * inner_bag_count, *inner_bag_name))
            });
    }
    count - 1
}
