type Input = (usize, usize, char, String);
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let mut range = parts.next().unwrap().split('-');
            let lhs = range.next().unwrap().parse().unwrap();
            let rhs = range.next().unwrap().parse().unwrap();
            let ch = parts.next().unwrap().chars().next().unwrap();
            let pwd = parts.next().unwrap().to_string();

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
                .chars()
                .filter(|c| c == ch)
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
            let is_lhs = pwd.chars().nth(*lhs - 1) == Some(*ch);
            let is_rhs = pwd.chars().nth(*rhs - 1) == Some(*ch);
            is_lhs != is_rhs
        })
        .count()
}
