use std::convert::TryFrom;
use itertools::Itertools;

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> usize {
    let groups = input
        .split(|&ch| ch == b'\n')
        .group_by(|v| v.is_empty());

    groups
        .into_iter()
        .map(|(_, group)| {
            let mut seen = [false; 26];
            group
                .into_iter()
                .flat_map(|v| v.iter())
                .for_each(|ch| {
                    let index = usize::try_from(ch - b'a').unwrap();
                    seen[index] = true;
                });
            seen
                .iter()
                .filter(|v| **v)
                .count()
        })
        .fold(0, |acc, v| acc + v)
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> usize {
    let groups = input
        .split(|&ch| ch == b'\n')
        .group_by(|v| v.is_empty());

    groups
        .into_iter()
        .map(|(_, group)| {
            let mut iter = group.into_iter();
            let mut common = iter.next().unwrap().to_vec();
            for other in iter {
                common.retain(|&ch| other.contains(&ch));
            }
            common.len()
        })
        .fold(0, |acc, v| acc + v)
}
