use std::collections::{
    HashMap,
    HashSet
};

use itertools::Itertools;

#[derive(Clone)]
struct Tile {
    field: Vec<u8>,
    w: usize
}

fn convert_to_binary(ch: &u8) -> u8 {
    if *ch == b'#' { b'1' } else { b'0' }
}

fn get_value(v: &[u8]) -> u16 {
    u16::from_str_radix(std::str::from_utf8(v).unwrap(), 2).unwrap()
}

// top right bottom left
type Hash = (u16, u16, u16, u16);
// eight rotations
type Hashes = [Hash; 8];
impl Tile {
    fn new(field: Vec<u8>) -> Tile {
        let w = (field.len() as f32).sqrt() as usize;
        Tile { field: field, w: w }
    }

    fn hash(&self) -> Hashes {
        let mut top_v = self.field[0..self.w].iter().map(convert_to_binary).collect_vec();
        let mut bottom_v = self.field[(self.w * self.w - self.w)..].iter().map(convert_to_binary).collect_vec();
        let mut left_v = (0..self.w).map(|y| convert_to_binary(&self.field[y * self.w])).collect_vec();
        let mut right_v = (1..=self.w).map(|y| convert_to_binary(&self.field[y * self.w - 1])).collect_vec();

        let top = get_value(&top_v);
        let bottom = get_value(&bottom_v);
        let left = get_value(&left_v);
        let right = get_value(&right_v);

        top_v.reverse();
        left_v.reverse();
        bottom_v.reverse();
        right_v.reverse();

        let top_r = get_value(&top_v);
        let bottom_r = get_value(&bottom_v);
        let left_r = get_value(&left_v);
        let right_r = get_value(&right_v);

        [
            (top, right, bottom, left),         // no change
            (right, bottom_r, left, top_r),     // rotate left
            (bottom_r, left_r, top_r, right_r), // rotate left x 2
            (left_r, top, right_r, bottom),     // rotate left x 3
            (bottom, right_r, top, left_r),     // flip against horizontal axis
            (top_r, left, bottom_r, right),     // flip against vertical axis
            (bottom_r, left_r, top_r, right_r), // both flips
            (right_r, top_r, left_r, bottom_r),     // rotate left -> flip against vertical axis
        ]
    }
}

type Input = HashMap<u64, Tile>;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Input {
    let groups = input.lines().group_by(|line| line.is_empty());

    groups
        .into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            let mut iter = group;
            let id = iter.next().unwrap()[5..9].parse().unwrap();
            let tile = iter.flat_map(|row| row.as_bytes().iter().cloned()).collect_vec();
            (id, Tile::new(tile))
        })
        .collect()
}

// edge value -> (id, rotation)
type LookupMap = HashMap<u16, Vec<(u64, usize)>>;
// id -> hashes, tops, lefts
type Indices = (HashMap<u64, Hashes>, LookupMap, LookupMap);

fn create_indices(input: &Input) -> Indices {
    let mut hashes = HashMap::new();
    let mut tops = HashMap::new();
    let mut lefts = HashMap::new();

    for (id, tile) in input.iter() {
        let h = tile.hash();

        for (rot, (t, _, _, l)) in h.iter().enumerate() {
            if !tops.contains_key(t) { tops.insert(*t, Vec::new()); }
            if !lefts.contains_key(l) { lefts.insert(*l, Vec::new()); }
            tops.get_mut(t).unwrap().push((*id, rot));
            lefts.get_mut(l).unwrap().push((*id, rot));
        }

        hashes.insert(*id, h);
    }

    (hashes, tops, lefts)
}

fn recurse(
    n: usize,
    indices: &Indices,
    visited: &mut HashSet<u64>,
    constructed: &mut Vec<(u64, usize)>
) -> bool {
    if constructed.len() == n * n { return true }

    let ref hashes = indices.0;
    let ref tops = indices.1;
    let ref lefts = indices.2;

    let top = constructed.len()
        .checked_sub(n)
        .map(|idx_top| {
            let (id, rot) = constructed[idx_top];
            hashes.get(&id).unwrap()[rot].2
        })
        .and_then(
            |top_hash| tops.get(&top_hash).map(
                |matches| matches.iter()
                    .filter(|(top_id, _)| !visited.contains(top_id))
                    .collect_vec()
            )
        );

    let left = (constructed.len() % n)
        .checked_sub(1)
        .map(|_| {
            let (id, rot) = constructed.last().unwrap();
            hashes.get(id).unwrap()[*rot].1
        })
        .and_then(
            |left_hash| lefts.get(&left_hash).map(
                |matches| matches.iter()
                    .filter(|(left_id, _)| !visited.contains(left_id))
                    .collect_vec()
            )
        );

    let candidates = match (top, left) {
        (Some(t), None) => t,
        (None, Some(l)) => l,
        (Some(t), Some(l)) => {
            let mut r = t;
            r.retain(|v| l.contains(v));
            r
        },
        _ => vec![]
    };

    for (id, rot) in candidates {
        constructed.push((*id, *rot));
        visited.insert(*id);

        if recurse(n, indices, visited, constructed) { return true }

        visited.remove(id);
        constructed.pop();
    }

    false
}

fn calc_part1(n: usize, input: &Input) -> Vec<(u64, usize)> {
    let indices = create_indices(input);

    let mut constructed = Vec::with_capacity(indices.0.len());
    let mut visited = HashSet::new();

    for (id, _) in input {
        for rot in 0..8 {
            constructed.push((*id, rot));
            visited.insert(*id);

            if recurse(n, &indices, &mut visited, &mut constructed) { return constructed }

            visited.remove(id);
            constructed.pop();
        }
    }

    panic!("Cannot find the solution");
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> u64 {
    let n = (input.len() as f32).sqrt() as usize;

    let constructed = calc_part1(n, input);
    constructed[0].0 * constructed[n-1].0 * constructed[n*n - 1].0 * constructed[n*n-n].0
}

impl Tile {
    fn cut_edges(&self) -> Tile {
        let w = self.w - 2;
        let mut field = Vec::with_capacity(w * w);
        for y in 1..=w {
            field.extend_from_slice(&self.field[(y * self.w + 1)..(y * self.w + w + 1)]);
        }
        Tile { w: w, field: field }
    }

    fn rotate(&self, rot: usize) -> Tile {
        match rot {
            0 => self.clone(),
            1 => {
                let field = iproduct!((0..self.w).rev(), (0..self.w))
                    .map(|(i, j)| self.field[j * self.w + i])
                    .collect_vec();
                Tile { w: self.w, field: field }
            },
            2 => {
                let field = iproduct!((0..self.w).rev(), (0..self.w).rev())
                    .map(|(i, j)| self.field[i * self.w + j])
                    .collect_vec();
                Tile { w: self.w, field: field }
            },
            3 => {
                let field = iproduct!((0..self.w), (0..self.w).rev())
                    .map(|(i, j)| self.field[j * self.w + i])
                    .collect_vec();
                Tile { w: self.w, field: field }
            },
            4 => {
                let mut field = Vec::with_capacity(self.field.capacity());
                for i in (0..self.w).rev() {
                    field.extend_from_slice(&self.field[(i*self.w)..(i*self.w + self.w)]);
                }
                Tile { w: self.w, field: field }
            },
            5 => {
                let mut field = Vec::with_capacity(self.field.capacity());
                for i in 0..self.w {
                    field.extend(self.field[(i*self.w)..(i*self.w + self.w)].iter().rev());
                }
                Tile { w: self.w, field: field }
            },
            6 => {
                let mut field = Vec::with_capacity(self.field.capacity());
                for i in (0..self.w).rev() {
                    field.extend(self.field[(i*self.w)..(i*self.w + self.w)].iter().rev());
                }
                Tile { w: self.w, field: field }
            },
            7 => {
                let field = iproduct!((0..self.w).rev(), (0..self.w).rev())
                    .map(|(i, j)| self.field[j * self.w + i])
                    .collect_vec();
                Tile { w: self.w, field: field }
            },
            _ => panic!("Wrong rotation!")
        }
    }
}

fn reconstruct(input: &Input, constructed: Vec<(u64, usize)>) -> Tile {
    let n = (input.len() as f32).sqrt() as usize;
    let input_w = input.get(
        &constructed.first().unwrap().0
    ).unwrap().w;
    let w = (input_w - 2) * n;

    let mut field = Vec::with_capacity(w * w);
    field.resize(w * w, 0);

    for (idx, (id, rot)) in constructed.iter().enumerate() {
        let tile = input.get(id).unwrap().cut_edges().rotate(*rot);

        let x = idx % n;
        let y = idx / n;

        for i in 0..tile.w {
            let start = (y * tile.w + i) * w + x * tile.w;
            let slice = &mut field[start..(start + tile.w)];
            slice.copy_from_slice(&tile.field[(i * tile.w)..(i * tile.w + tile.w)]);
        }
    }

    Tile { field: field, w: w }
}

impl Tile {
    const POSITIONS: [(usize, usize); 15] = [
        (18, 0),
        (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
        (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)
    ];

    fn check_monster(&self, (y, x): &(usize, usize)) -> bool {
        x + 19 < self.w && y + 2 < self.w &&
            Tile::POSITIONS.iter().all(|(i, j)| self.field[(y + j) * self.w + x + i] == b'#')
    }

    fn mark_monster(&mut self, (y, x): &(usize, usize)) {
        for (i, j) in Tile::POSITIONS.iter() {
            self.field[(y+j)*self.w + x + i] = b'O';
        }
    }

    fn verify_image(&mut self) -> bool {
        let mut seen_monster = false;

        for coord in iproduct!(0..(self.w - 2), 0..(self.w-19)) {
            if self.check_monster(&coord) {
                self.mark_monster(&coord);
                seen_monster = true;
            }
        }

        seen_monster
    }

    fn find_final_rotation(&self) -> Tile {
        for rot in 0..4 {
            let mut rotated = self.rotate(rot);
            if rotated.verify_image() {
                return rotated;
            }

            let mut flipped_h = rotated.rotate(4);
            if flipped_h.verify_image() {
                return flipped_h;
            }

            let mut flipped_v = rotated.rotate(5);
            if flipped_v.verify_image() {
                return flipped_v;
            }

            let mut flipped_hv = flipped_h.rotate(5);
            if flipped_hv.verify_image() {
                return flipped_hv;
            }
        }

        panic!("Couldn't find the solution!");
    }
    //
    // pub fn print(&self) {
    //     for i in 0..self.w {
    //         println!("{}", std::str::from_utf8(&self.field[(i * self.w)..(i * self.w + self.w)]).unwrap());
    //     }
    //     println!("");
    // }
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> usize {
    let n = (input.len() as f32).sqrt() as usize;

    let constructed = calc_part1(n, input);
    let image = reconstruct(input, constructed);

    let final_rotation = image.find_final_rotation();

    final_rotation.field.iter()
        .filter(|&ch| *ch == b'#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_works() {
        let tile = Tile::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let cut = tile.cut_edges();
        assert_eq!(cut.w, 1);
        assert_eq!(cut.field, vec![5]);
    }

    #[test]
    fn rotate_0() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(0);
        assert_eq!(rotated.field, tile.field);

        assert_eq!(tile.hash()[0], rotated.hash()[0]);
    }

    #[test]
    fn rotate_1() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(1);
        assert_eq!(rotated.field, vec![
            b'.', b'#', b'#',
            b'.', b'#', b'.',
            b'.', b'.', b'#'
        ]);
        assert_eq!(tile.hash()[1], rotated.hash()[0]);
    }

    #[test]
    fn rotate_2() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(2);
        assert_eq!(rotated.field, vec![
            b'#', b'.', b'#',
            b'#', b'#', b'.',
            b'.', b'.', b'.'
        ]);
        assert_eq!(tile.hash()[2], rotated.hash()[0]);
    }

    #[test]
    fn rotate_3() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(3);
        assert_eq!(rotated.field, vec![
            b'#', b'.', b'.',
            b'.', b'#', b'.',
            b'#', b'#', b'.'
        ]);
        assert_eq!(tile.hash()[3], rotated.hash()[0]);
    }

    #[test]
    fn rotate_4() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(4);
        assert_eq!(rotated.field, vec![
            b'#', b'.', b'#',
            b'.', b'#', b'#',
            b'.', b'.', b'.'
        ]);
        assert_eq!(tile.hash()[4], rotated.hash()[0]);
    }

    #[test]
    fn rotate_5() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(5);
        assert_eq!(rotated.field, vec![
            b'.', b'.', b'.',
            b'#', b'#', b'.',
            b'#', b'.', b'#'
        ]);
        assert_eq!(tile.hash()[5], rotated.hash()[0]);
    }

    #[test]
    fn rotate_6() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(6);
        assert_eq!(rotated.field, vec![
            b'#', b'.', b'#',
            b'#', b'#', b'.',
            b'.', b'.', b'.'
        ]);
        assert_eq!(tile.hash()[6], rotated.hash()[0]);
    }

    #[test]
    fn rotate_7() {
        let tile = Tile::new(vec![
            b'.', b'.', b'.',
            b'.', b'#', b'#',
            b'#', b'.', b'#'
        ]);
        let rotated = tile.rotate(7);

        assert_eq!(rotated.field, vec![
            b'#', b'#', b'.',
            b'.', b'#', b'.',
            b'#', b'.', b'.'
        ]);
        assert_eq!(tile.hash()[7], rotated.hash()[0]);
    }
}
