use std::collections::HashSet;
use std::iter::FromIterator;

type Coord = (i8, i8, i8, i8);
type Edges = (Coord, Coord);
type Field = (HashSet<Coord>, Edges, usize);

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Field {
    (
        HashSet::from_iter(
            input.lines().enumerate()
                .flat_map(|(y, line)|
                    line.as_bytes().iter().enumerate()
                        .filter(|(_, &ch)| ch == b'#')
                        .map(move |(x, _)| (x as i8, y as i8, 0, 0))
                )
        ),
        (
            (0, 0, 0, 0),
            (
                input.lines().count() as i8,
                input.lines().next().unwrap().as_bytes().len() as i8,
                0,
                0
            )
        ),
        0
    )
}

fn create_range(v: i8, min: i8, max: i8) -> std::ops::RangeInclusive<i8> {
    std::ops::RangeInclusive::new(
        min.max(v - 1),
        max.min(v + 1)
    )
}

fn neighbours_3d((space, (min, max), _): &Field, center: Coord) -> usize {
    let mut ans = 0;
    for coord in iproduct!(
        create_range(center.0, min.0, max.0),
        create_range(center.1, min.1, max.1),
        create_range(center.2, min.2, max.2),
        0..=0
    ) {
        if coord == center { continue }
        if !space.contains(&coord) { continue }
        ans += 1
    }
    ans
}

fn evolve_3d(field: &Field) -> Field {
    let mut result = HashSet::new();
    let mut minmax = (
        (i8::MAX, i8::MAX, i8::MAX, 0),
        (i8::MIN, i8::MIN, i8::MIN, -1)
    );
    let mut count = 0;
    for coord in iproduct!(
        (field.1.0.0-1)..=(field.1.1.0+1),
        (field.1.0.1-1)..=(field.1.1.1+1),
        (field.1.0.2-1)..=(field.1.1.2+1),
        0..=0
    ) {
        match (field.0.contains(&coord), neighbours_3d(field, coord)) {
            (true, 2..=3) | (false, 3) => {
                count += 1;
                result.insert(coord);
                minmax.0.0 = minmax.0.0.min(coord.0);
                minmax.0.1 = minmax.0.1.min(coord.1);
                minmax.0.2 = minmax.0.2.min(coord.2);

                minmax.1.0 = minmax.1.0.max(coord.0);
                minmax.1.1 = minmax.1.1.max(coord.1);
                minmax.1.2 = minmax.1.2.max(coord.2);
            },
            _ => {}
        }
    }

    (result, minmax, count)
}

#[aoc(day17, part1)]
fn part1(input: &Field) -> usize {
    let mut field = input.clone();
    for _ in 0..6 {
        field = evolve_3d(&field);
    }
    field.2
}

fn neighbours_4d((space, (min, max), _): &Field, center: Coord) -> usize {
    let mut ans = 0;
    for coord in iproduct!(
        create_range(center.0, min.0, max.0),
        create_range(center.1, min.1, max.1),
        create_range(center.2, min.2, max.2),
        create_range(center.3, min.3, max.3)
    ) {
        if coord == center { continue }
        if !space.contains(&coord) { continue }
        ans += 1
    }
    ans
}

fn evolve_4d(field: &Field) -> Field {
    let mut result = HashSet::new();
    let mut minmax = (
        (i8::MAX, i8::MAX, i8::MAX, i8::MAX),
        (i8::MIN, i8::MIN, i8::MIN, i8::MIN)
    );
    let mut count = 0;
    for coord in iproduct!(
        (field.1.0.0-1)..=(field.1.1.0+1),
        (field.1.0.1-1)..=(field.1.1.1+1),
        (field.1.0.2-1)..=(field.1.1.2+1),
        (field.1.0.3-1)..=(field.1.1.3+1)
    ) {
        match (field.0.contains(&coord), neighbours_4d(field, coord)) {
            (true, 2..=3) | (false, 3) => {
                count += 1;
                result.insert(coord);
                minmax.0.0 = minmax.0.0.min(coord.0);
                minmax.0.1 = minmax.0.1.min(coord.1);
                minmax.0.2 = minmax.0.2.min(coord.2);
                minmax.0.3 = minmax.0.3.min(coord.3);

                minmax.1.0 = minmax.1.0.max(coord.0);
                minmax.1.1 = minmax.1.1.max(coord.1);
                minmax.1.2 = minmax.1.2.max(coord.2);
                minmax.1.3 = minmax.1.3.max(coord.3);
            },
            _ => {}
        }
    }

    (result, minmax, count)
}

#[aoc(day17, part2)]
fn part2(input: &Field) -> usize {
    let mut field = input.clone();
    for _ in 0..6 {
        field = evolve_4d(&field);
    }
    field.2
}
