use std::io::BufRead;
use std::convert::TryFrom;
use std::time::{Instant};
use std::io;

type FreqMap = [bool; 2020];

fn main() {
    let reading_timer = Instant::now();
    let (numbers, freqs) = read_input();
    println!("Read in {}μs", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    println!("Part 1 is {}, took {}μs", part1(&numbers, freqs), part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}μs", part2(&numbers, freqs), part2_timer.elapsed().as_micros());
}

fn part1(numbers: &[i32], freqs: FreqMap) -> i32 {
    for num in numbers.iter() {
        let other = 2020 - num;
        if freqs[usize::try_from(other).unwrap()] {
            return num * other;
        }
    }
    panic!("Couldn't find answer!");
}

fn part2(numbers: &[i32], freqs: FreqMap) -> i32 {
    for (i, a) in numbers.iter().enumerate() {
        for b in numbers[i+1..].iter() {
            let other = 2020 - a - b;
            if other < 0 {
                break;
            }
            if freqs[usize::try_from(other).unwrap()] {
                return a * b * other;
            }
        }
    }
    panic!("Couldn't find answer!");
}

fn read_input() -> (Vec<i32>, FreqMap) {
    let mut freqs = [false; 2020];
    let mut result = Vec::new();
    for line in io::stdin().lock().lines() {
        if let Ok(l) = line {
            let num: i32 = l.parse().unwrap();
            result.push(num);
            freqs[usize::try_from(num).unwrap()] = true;
        }
    }

    result.sort();

    (result, freqs)
}
