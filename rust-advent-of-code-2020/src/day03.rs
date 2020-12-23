fn check_slope(input: &[u8], x: usize, y: usize) -> usize {
    input
        .split(|&ch| ch == b'\n')
        .step_by(y)
        .enumerate()
        .filter(|(i, s)| s.get(x * i % s.len()) == Some(&b'#'))
        .count()
}

#[aoc(day3, part1)]
pub fn part1(input: &[u8]) -> usize {
    check_slope(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &[u8]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (x, y)| acc * check_slope(input, *x, *y))
}
