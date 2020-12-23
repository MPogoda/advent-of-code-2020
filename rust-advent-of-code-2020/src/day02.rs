use std::str;

type Input = (usize, usize, u8, Vec<u8>);
#[aoc_generator(day2)]
pub fn input_generator(input: &[u8]) -> Vec<Input> {
    input
        .split(|&ch| ch == b'\n')
        .map(|line| {
            let mut parts = line.split(|&ch| ch == b' ');
            let mut range = parts.next().unwrap().split(|&ch| ch == b'-');
            let lhs = str::from_utf8(range.next().unwrap()).unwrap().parse().unwrap();
            let rhs = str::from_utf8(range.next().unwrap()).unwrap().parse().unwrap();
            let ch = *parts.next().unwrap().get(0).unwrap();
            let pwd = parts.next().unwrap().to_vec();

            (lhs, rhs, ch, pwd)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|(min, max, ch, pwd)| {
            let count = pwd
                .iter()
                .filter(|&c| c == ch)
                .count();
            count >= *min && count <= *max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Input]) -> usize {
    input
        .iter()
        .filter(|(lhs, rhs, ch, pwd)| {
            let is_lhs = pwd.get(*lhs - 1) == Some(ch);
            let is_rhs = pwd.get(*rhs - 1) == Some(ch);
            is_lhs != is_rhs
        })
        .count()
}
