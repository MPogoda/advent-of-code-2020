use itertools::Itertools;

#[aoc_generator(day25)]
fn parse_input(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

#[aoc(day25, part1)]
fn part1((key_pub, door_pub): &(u64, u64)) -> u64 {
    let mut c = 7;
    let mut count = 1;

    let mut key_priv = 0;
    let mut door_priv = 0;
    while key_priv == 0 && door_priv == 0 {
        c *= 7;
        c %= 20201227;
        count += 1;
        if c == *key_pub {
            key_priv = count;
        }
        if c == *door_pub {
            door_priv = count;
        }
    }

    count = if key_priv == 0 { door_priv } else { key_priv };
    let mult = if key_priv == 0 { *key_pub } else { *door_pub };

    let mut ans = 1;
    while count > 0 {
        ans *= mult;
        ans %= 20201227;
        count -= 1;
    }

    ans
}
