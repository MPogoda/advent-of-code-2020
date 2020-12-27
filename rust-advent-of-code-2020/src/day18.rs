enum Val {
    Num(u64),
    Plus,
    Times,
    Par
}

impl Val {
    fn unwrap(&self) -> &u64 {
        match self {
            Val::Num(val) => val,
            _ => { panic!("Cannot unwrap non-number"); }
        }
    }
}

fn solve1(line: &str) -> u64 {
    let mut acc = Vec::new();
    for ch in line.as_bytes() {
        match ch {
            b'0'..=b'9' => {
                let this = (ch - b'0') as u64;
                match acc.last() {
                    Some(Val::Plus) => {
                        acc.pop();
                        let prev = acc.pop().unwrap();
                        acc.push(Val::Num(prev.unwrap() + this))
                    },
                    Some(Val::Times) => {
                        acc.pop();
                        let prev = acc.pop().unwrap();
                        acc.push(Val::Num(prev.unwrap() * this))
                    },
                    _ => { acc.push(Val::Num(this)) }
                }
            },
            b'(' => { acc.push(Val::Par) },
            b')' => {
                let val = acc.pop().unwrap();
                acc.pop();

                match acc.last() {
                    Some(Val::Plus) => {
                        acc.pop();
                        let prev = acc.pop().unwrap();
                        acc.push(Val::Num(prev.unwrap() + val.unwrap()));
                    },
                    Some(Val::Times) => {
                        acc.pop();
                        let prev = acc.pop().unwrap();
                        acc.push(Val::Num(prev.unwrap() * val.unwrap()));
                    },
                    _ => acc.push(val)
                }
            },
            b'+' => { acc.push(Val::Plus); },
            b'*' => { acc.push(Val::Times); },
            b' ' => {},
            _ => { panic!("Hm"); }
        }
    }

    *acc[0].unwrap()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| solve1(line))
        .sum()
}

fn eval(acc: &mut Vec<Val>) {
    let mut ans = 1;
    while !acc.is_empty() {
        match acc.pop().unwrap() {
            Val::Par => { break; },
            Val::Times => {
                let prev = acc.pop().unwrap();
                ans *= prev.unwrap();
            },
            Val::Num(prev) => { ans *= prev; },
            _ => { panic!("unexpected plus!"); }
        }
    }
    acc.push(Val::Num(ans));
}

fn solve2(line: &str) -> u64 {
    let mut acc = Vec::new();
    for ch in line.as_bytes() {
        match ch {
            b' ' => {},
            b'0'..=b'9' => {
                let this = (ch - b'0') as u64;
                match acc.last() {
                    Some(Val::Plus) => {
                        acc.pop();
                        let prev = acc.pop().unwrap();
                        acc.push(Val::Num(prev.unwrap() + this))
                    },
                    _ => { acc.push(Val::Num(this)); }
                }
            },
            b'(' => { acc.push(Val::Par) },
            b')' => {
                eval(&mut acc);
                let val = acc.pop().unwrap();

                match acc.last() {
                    Some(Val::Plus) => {
                        acc.pop();
                        let prev = acc.pop().unwrap();
                        acc.push(Val::Num(prev.unwrap() + val.unwrap()));
                    },
                    _ => acc.push(val)
                }
            },
            b'+' => { acc.push(Val::Plus); },
            b'*' => { acc.push(Val::Times); },
            _ => { panic!("Hm"); }
        }
    }
    eval(&mut acc);

    *acc[0].unwrap()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| solve2(line))
        .sum()
}
