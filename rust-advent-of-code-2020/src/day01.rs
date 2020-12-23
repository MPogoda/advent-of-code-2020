use std::str;

type SeenMap = [bool; 2020];
type Input = (Vec<usize>, SeenMap);

#[aoc_generator(day1)]
pub fn input_generator(input: &[u8]) -> Input {
    let mut result: Vec<_> = input
        .split(|&ch| ch == b'\n')
        .map(|l| str::from_utf8(l).unwrap().parse().unwrap())
        .collect();

    result.sort_unstable();

    let mut seen = [false; 2020];
    for &num in &result {
        seen[num] = true;
    }

    (result, seen)
}

#[aoc(day1, part1)]
pub fn part1((numbers, seen): &Input) -> usize {
    for num in numbers {
        let other = 2020 - num;
        if seen[other] {
            return other * num;
        }
    }
    panic!("Couldn't find answer");
}

#[aoc(day1, part2)]
pub fn part2((numbers, seen): &Input) -> usize {
    for (i, a) in numbers.iter().enumerate() {
        for b in numbers[(i+1)..].iter() {
            if a + b >= 2020 { break }
            let other = 2020 - a - b;
            if seen[other] { return a * b * other; }
        }
    }
    panic!("Couldn't find answer");
}
