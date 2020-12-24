use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Instr {ACC, JMP, NOP }
pub type Cmd = (Instr, i32);

#[aoc_generator(day8)]
pub fn parse_input(input: &[u8]) -> Vec<Cmd> {
    input
        .split(|&ch| ch == b'\n')
        .map(|line| {
            let (lhs, rhs) = line.split_at(4);
            (
                match lhs {
                    b"acc " => Instr::ACC,
                    b"jmp " => Instr::JMP,
                    _ => Instr::NOP
                },
                std::str::from_utf8(rhs).unwrap().parse().unwrap()
            )
        })
        .collect()
}

fn run(input: &[Cmd]) -> (i32, HashSet<usize>) {
    let mut result = 0i32;
    let mut visited = HashSet::new();

    let mut ip = 0;
    loop {
        if visited.contains(&ip) { break }
        visited.insert(ip);
        if ip >= input.len() { break }
        let (instr, param) = input.get(ip).unwrap();

        match instr {
            Instr::ACC => {
                result += param;
                ip += 1;
            },
            Instr::NOP => {
                ip += 1;
            },
            Instr::JMP => {
                ip = ip.wrapping_add(*param as usize);
            }
        }
    }

    (result, visited)
}

#[aoc(day8, part1)]
pub fn part1(input: &[Cmd]) -> i32 {
    run(input).0
}

#[aoc(day8, part2)]
fn part2(input: &[Cmd]) -> i32 {
    let mut program = input.to_vec();

    let culprits = run(&program).1;

    for culprit in culprits {
        match program[culprit].0 {
            Instr::JMP => {
                program[culprit].0 = Instr::NOP;
                let (result, visited) = run(&program);
                if visited.contains(&program.len()) {
                    return result;
                }
                program[culprit].0 = Instr::JMP;
            },
            Instr::NOP => {
                program[culprit].0 = Instr::JMP;
                let (result, visited) = run(&program);
                if visited.contains(&program.len()) {
                    return result;
                }
                program[culprit].0 = Instr::NOP;
            },
            Instr::ACC => {}
        }
    }
    panic!("Cannot find solution!");
}
