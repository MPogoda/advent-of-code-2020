use std::collections::HashSet;

type Coord = (i16, i16);

#[derive(Clone)]
struct Field {
    field: HashSet<Coord>,
    min: Coord,
    max: Coord
}

impl Field {
    fn new() -> Field {
        Field {
            field: HashSet::new(),
            min: (i16::MAX, i16::MAX),
            max: (i16::MIN, i16::MIN)
        }
    }

    fn update_min_max(&mut self, coord: &Coord) {
        self.min.0 = self.min.0.min(coord.0);
        self.min.1 = self.min.1.min(coord.1);

        self.max.0 = self.max.0.max(coord.0);
        self.max.1 = self.max.1.max(coord.1);
    }

    fn parse_line(&mut self, line: &str) {
        let mut coord = (0, 0);
        let mut prev_vertical = false;
        for ch in line.as_bytes() {
            match ch {
                b'n' | b's' => {
                    coord.0 += if ch == &b'n' { 2 } else { -2 };
                    prev_vertical = true;
                }
                b'e' | b'w' => {
                    let mult = if prev_vertical { 1 } else { 2 };
                    coord.1 += mult * if ch == &b'e' { 1 } else { -1 };
                    prev_vertical = false;
                },
                _ => panic!("Unexpected direction!")
            }
        }

        self.flip(coord)
    }

    fn flip(&mut self, coord: Coord) {
        self.update_min_max(&coord);
        if !self.field.insert(coord) {
            self.field.remove(&coord);
        }
    }

    fn flipped(&self) -> usize {
        self.field.len()
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Field {
    let mut field = Field::new();
    for line in input.lines() {
        field.parse_line(line);
    }
    field
}

#[aoc(day24, part1)]
fn part1(input: &Field) -> usize {
    input.flipped()
}

impl Field {
    const NEIGHBOURS_DIFFS: [Coord; 6] = [
        (0, 2), (0, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)
    ];
    fn neighbours(&self, (n, e): &Coord) -> usize {
        Field::NEIGHBOURS_DIFFS.iter()
            .filter(|(dn, de)| self.field.contains(&(n + dn, e + de)))
            .count()
    }

    fn evolve(&self) -> Field {
        let mut result = Field::new();

        for coord in iproduct!(
            ((self.min.0 - 2)..=(self.max.0 + 2)).step_by(2),
            (self.min.1 - 2)..=(self.max.1 + 2)
        ) {
            let is_set = self.field.contains(&coord);
            let neighbours = self.neighbours(&coord);
            match (is_set, neighbours) {
                (false, 2) | (true, 1) | (true, 2) => {
                    result.flip(coord);
                },
                _ => {}
            }
        }
        result
    }
}

#[aoc(day24, part2)]
fn part2(input: &Field) -> usize {
    let mut field = input.clone();
    for _ in 0..100 {
        field = field.evolve();
    }

    field.flipped()
}
