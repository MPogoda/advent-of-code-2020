use itertools::Itertools;
use std::collections::VecDeque;

#[aoc_generator(day9)]
fn parse_input(input: &[u8]) -> Vec<i64> {
    input
        .split(|&ch| ch == b'\n')
        .map(|line| std::str::from_utf8(line).unwrap().parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[i64]) -> i64 {
    let mut iter = input.iter();
    let mut window: VecDeque<_> = (&mut iter).take(25).collect();

    for n in iter {
        if (&window)
            .iter()
            .find(|&x| (&window).contains(&&(n - *x)))
            .is_none()
        {
            return *n;
        }
        window.pop_front();
        window.push_back(&n);
    }

    panic!("Cannot find solution!");
}

#[aoc(day9, part2)]
fn part2(input: &[i64]) -> i64 {
    let bad_num = part1(input);

    let mut list = VecDeque::new();
    let mut sum = 0;

    for i in input {
        list.push_back(i);
        sum += i;

        while !list.is_empty() && sum > bad_num {
            sum -= list.pop_front().unwrap();
        }
        if sum == bad_num && list.len() > 1 {
            break;
        }
    }

    let (&min, &max) = list.iter().minmax().into_option().unwrap();
    return min + max;
}
