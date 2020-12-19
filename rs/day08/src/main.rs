use std::io::{self, BufRead};
use std::time::{Instant};
use std::collections::HashSet;

fn main() {
    let reading_timer = Instant::now();
    let input = read_input();
    println!("Read in {}μs", reading_timer.elapsed().as_micros());

    let part1_timer = Instant::now();
    println!("Part 1 is {}, took {}μs", part1(&input), part1_timer.elapsed().as_micros());

    let part2_timer = Instant::now();
    println!("Part 2 is {}, took {}μs", part2(&input), part2_timer.elapsed().as_micros());
}

#[derive(Clone, Debug)]
enum Instr { ACC, JMP, NOP }
type Cmd = (Instr, i32);

fn read_input() -> Vec<Cmd> {
    io::stdin()
        .lock()
        .lines()
        .filter(|v| v.is_ok())
        .map(|v| {
            let str = v.unwrap();
            let (lhs, rhs) = str.split_at(4);
            (
                match lhs {
                    "acc " => Instr::ACC,
                    "jmp " => Instr::JMP,
                    _ => Instr::NOP
                },
                rhs.parse().unwrap()
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

fn part1(input: &[Cmd]) -> i32 {
    run(input).0
}

fn part2(input: &[Cmd]) -> i32 {
    let mut program = Vec::with_capacity(input.len());
    program.extend_from_slice(input);

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

