use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

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

fn read_input() -> Vec<i64> {
    io::stdin()
        .lock()
        .lines()
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap().parse().unwrap())
        .collect()
}

fn part1(input: &[i64]) -> i64 {
    input
        .windows(26)
        .find(|vs| {
            let num = vs[25];
            let nums: HashSet<i64> = vs.iter().take(25).map(|v| *v).collect();
            let subs: HashSet<i64> = nums.iter().map(|v| num - *v).collect();

            nums.intersection(&subs).next().is_none()
        }).unwrap()[25]
}

fn part2(input: &[i64], bad_num: i64) -> i64 {
    let mut list = VecDeque::new();
    let mut sum = 0;

    for i in input {
        list.push_back(i);
        sum += i;

        while !list.is_empty() && sum > bad_num {
            sum -= list.pop_front().unwrap();
        }
        if sum == bad_num && list.len() > 1 { break }
    }

    if let MinMax(&min, &max) = list.iter().minmax() {
        return min + max;
    }
    panic!("Cannot find solution!");
}
