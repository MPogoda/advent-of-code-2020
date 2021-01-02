#[aoc_generator(day23)]
fn parse_input(input: &[u8]) -> Vec<usize> {
    input.iter().map(|ch| (ch - b'0') as usize).collect()
}

fn perform_move(arr: &mut [usize], n: usize, current: &mut usize) {
    let c0 = arr[*current];
    let c1 = arr[c0];
    let c2 = arr[c1];
    let mut target = if *current > 1 { *current - 1 } else { n };
    while target == c0 || target == c1 || target == c2 {
        target = if target > 1 { target - 1 } else { n };
    }

    arr[*current] = arr[c2];
    *current = arr[*current];

    arr[c2] = arr[target];
    arr[target] = c0;
}

#[aoc(day23, part1)]
fn part1(input: &[usize]) -> usize {
    let n = 9;
    let mut arr = Vec::with_capacity(n + 1);
    arr.resize(n + 1, 0);
    for i in 0..(input.len() - 1) {
        arr[input[i]] = input[i + 1];
    }
    arr[input[input.len() - 1]] = input[0];
    let mut current = input[0];

    for _ in 0..100 {
        perform_move(&mut arr, n, &mut current);
    }

    let mut result = 0;
    current = 1;
    let mut power = 10_000_000;
    while arr[current] != 1 {
        result += arr[current] * power;
        power /= 10;
        current = arr[current];
    }

    result
}

#[aoc(day23, part2)]
fn part2(input: &[usize]) -> usize {
    let n = 1_000_000;
    let mut arr = Vec::with_capacity(n + 1);
    for i in 0..n {
        arr.push(i + 1);
    }
    arr.push(input[0]);
    for i in 0..(input.len() - 1) {
        arr[input[i]] = input[i + 1];
    }
    arr[input[input.len() - 1]] = input.len() + 1;
    let mut current = input[0];

    for _ in 0..10_000_000 {
        perform_move(&mut arr, n, &mut current);
    }

    arr[1] * arr[arr[1]]
}
