use std::iter::FromIterator;
use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::{VecDeque, HashMap, HashSet};
use regex::Regex;

const DESIRED_BAG: &str = "shiny gold";

fn main() {
    let reading_timer = Instant::now();
    let input = read_input();
    println!("Read in {}ms", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    println!("Part 1 is {}, took {}ms", part1(&input), part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}ms", part2(&input), part2_timer.elapsed().as_micros());
}

type Dep = (String, usize);
type Map = HashMap<String, Vec<Dep>>;

fn read_input() -> Map {
    let re = Regex::new(r"(?P<count>\d+) (?P<name>[^,]+) bag").unwrap();

    Map::from_iter(
        io::stdin()
            .lock()
            .lines()
            .filter(|v| v.is_ok())
            .map(|v| {
                let string = v.unwrap();
                let splits : Vec<_> = string.split(" bags contain ").collect();
                assert_eq!(splits.len(), 2);

                let name = splits[0].to_string();
                if &splits[1] == &"no other bags." {
                    return (name, vec![]);
                }

                (name, re.captures_iter(splits[1]).map(|cap| (
                            cap["name"].to_string(),
                            cap["count"].parse::<usize>().unwrap())
                ).collect())
            })
    )
}

fn part1(input: &Map) -> usize {
    let mut deps = HashMap::<&str, Vec<&str>>::new();
    input
        .iter()
        .flat_map(|(outer, deps)| deps.iter().map(move |(dep, _)| (dep.as_str(), outer.as_str())))
        .for_each(|(inner, outer)| {
            if !deps.contains_key(inner) {
                deps.insert(inner, Vec::new());
            }
            deps.get_mut(inner).unwrap().push(outer);
        });

    let mut visited = HashSet::<&str>::new();
    let mut queue = VecDeque::new();
    queue.push_back(DESIRED_BAG);
    while !queue.is_empty() {
        let this_bag = queue.pop_front().unwrap();
        visited.insert(this_bag);
        deps.get(this_bag)
            .iter()
            .flat_map(|vs| vs.iter())
            .for_each(|dep| if !visited.contains(*dep) {
                queue.push_back(dep);
            });
    }
    visited.len() - 1
}

fn part2(input: &Map) -> usize {
    let mut stack = Vec::new();
    let mut count = 0usize;
    stack.push((1usize, DESIRED_BAG));
    while !stack.is_empty() {
        let (this_bag_count, this_bag_name) = stack.pop().unwrap();
        count += this_bag_count;
        input.get(this_bag_name)
            .unwrap()
            .iter()
            .for_each(|(inner_bag_name, inner_bag_count)| stack.push(
                    (this_bag_count * inner_bag_count, inner_bag_name)
            ));
    }
    count - 1
}

