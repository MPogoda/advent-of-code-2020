#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(|ch| ch == ',')
        .map(|val| val.parse().unwrap())
        .collect()
}

fn go(input: &[usize], n: usize) -> usize {
    let mut seen = Vec::with_capacity(n);
    seen.resize(n, usize::MAX);
    for (i, v) in input.iter().enumerate() {
        seen[*v] = i;
    }
    let mut arr = Vec::with_capacity(n);
    arr.extend_from_slice(input);
    while arr.len() < n {
        let prev = *arr.last().unwrap();
        if seen[prev] == usize::MAX {
            arr.push(0);
        } else {
            let next = arr.len() - 1 - seen[prev];
            arr.push(next);
        }
        seen[prev] = arr.len() - 2;
    }
    *arr.last().unwrap()
}

#[aoc(day15, part1)]
fn part1(input: &[usize]) -> usize {
    go(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &[usize]) -> usize {
    go(input, 30_000_000)
}
