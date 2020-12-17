use std::io::{self, BufRead};
use std::time::{Instant};

type Line = (usize, usize, char, String);

fn main() {
    let reading_timer = Instant::now();
    let input = read_input();
    println!("Read in {}ms", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    println!("Part 1 is {}, took {}ms", part1(&input), part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}ms", part2(&input), part2_timer.elapsed().as_micros());
}

fn part1(input: &[Line]) -> usize {
    let mut ans = 0;
    for (min, max, ch, pwd) in input {
        let count = pwd.chars().filter(|c| c == ch).count();
        if count >= *min && count <= *max {
            ans += 1;
        }
    }
    ans
}

fn part2(input: &[Line]) -> usize {
    let mut ans = 0;
    for (lhs, rhs, ch, pwd) in input {
        let is_lhs = pwd.chars().nth(*lhs - 1) == Some(*ch);
        let is_rhs = pwd.chars().nth(*rhs - 1) == Some(*ch);
        if is_lhs != is_rhs {
            ans += 1;
        }
    }
    ans
}

fn read_input() -> Vec<Line> {
    let mut result = Vec::new();
    for line in io::stdin().lock().lines() {
        if let Ok(l) = line {
            let parts: Vec<_> = l.split(' ').collect();

            let range: Vec<_> = parts.get(0).unwrap().split('-').collect();
            let min = range.get(0).unwrap().parse().unwrap();
            let max = range.get(1).unwrap().parse().unwrap();

            let ch = parts.get(1).unwrap().chars().next().unwrap();

            let pwd = parts.get(2).unwrap();

            result.push((min, max, ch, pwd.to_string()));
        }
    }

    result
}
