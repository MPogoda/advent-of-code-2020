type Input = (u64, Vec<Option<u64>>);

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let moment = lines.next().unwrap().parse().unwrap();
    let nums: Vec<_> = lines
        .next()
        .unwrap()
        .split(|ch| ch == ',')
        .map(|num| match num.parse() {
            Ok(v) => Some(v),
            _ => None
        })
        .collect();
    ( moment, nums )
}

#[aoc(day13, part1)]
fn part1((moment, timetable): &Input) -> u64 {
    let bus = timetable
        .iter()
        .filter(|num| num.is_some())
        .map(|num| num.unwrap())
        .min_by(|lhs, rhs| {
            let lhs_cmp = lhs - (moment % lhs);
            let rhs_cmp = rhs - (moment % rhs);
            lhs_cmp.cmp(&rhs_cmp)
        })
        .unwrap();

    bus * (bus - (moment % bus))
}

#[aoc(day13, part2)]
fn part2((_, timetable): &Input) -> u64 {
    timetable
        .iter()
        .enumerate()
        .filter(|(_, bus)| bus.is_some())
        .map(|(i, bus)| (i, bus.unwrap()))
        .fold((0, 1), |(t, step), (i, bus)| {
            let want = (100 * bus - i as u64) % bus;
            let mut next_t = t;
            while next_t % bus != want { next_t += step; }
            (next_t, step * bus)
        }).0
}
