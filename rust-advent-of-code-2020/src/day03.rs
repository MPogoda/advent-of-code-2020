fn check_slope(input: &str, x: usize, y: usize) -> usize {
    input
        .lines()
        .step_by(y)
        .enumerate()
        .filter(|(i, s)| s.chars().nth(x * i % s.len()) == Some('#'))
        .count()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    check_slope(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (x, y)| acc * check_slope(input, *x, *y))
}
