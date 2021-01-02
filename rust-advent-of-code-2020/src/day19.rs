use itertools::Itertools;
use std::collections::VecDeque;

type RuleSet = Vec<usize>;
type Line = Vec<u8>;
#[derive(Clone)]
enum Rule {
    Verbatim(u8),
    Recursive(Vec<RuleSet>),
}

type Input = (Vec<Rule>, Vec<Line>);

fn parse_rule(line: &str) -> (usize, Rule) {
    let mut parts = line.split(": ");
    let id = parts.next().unwrap().parse().unwrap();

    let rhs = parts.next().unwrap();
    if rhs.starts_with('"') {
        (id, Rule::Verbatim(rhs.as_bytes()[1]))
    } else {
        let mut rulesets = Vec::new();
        for part in rhs.split('|') {
            rulesets.push(
                part.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|num| num.parse().unwrap())
                    .collect(),
            );
        }
        (id, Rule::Recursive(rulesets))
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Input {
    let groups = input.lines().group_by(|line| line.is_empty());
    let mut groups_iter = groups.into_iter();

    let mut rules = Vec::with_capacity(200);
    rules.resize(200, Rule::Verbatim(0));

    for line in groups_iter.next().unwrap().1 {
        let (id, rule) = parse_rule(line);
        rules[id] = rule;
    }

    groups_iter.next();

    let lines = groups_iter
        .next()
        .unwrap()
        .1
        .map(|line| line.as_bytes().to_vec())
        .collect();

    (rules, lines)
}

fn non_recursive_verify_subrule(line: &[u8], rules: &[Rule], subrule: &RuleSet) -> usize {
    let mut matched = 0;
    for p in subrule {
        let matched_length = non_recursive_verify(&line[matched..], rules, &rules[*p]);
        matched += matched_length;
        if matched == 0 || matched_length > line.len() {
            return 0;
        }
    }
    matched
}

fn non_recursive_verify(line: &[u8], rules: &[Rule], rule: &Rule) -> usize {
    match rule {
        Rule::Verbatim(ch) => {
            if line.first() == Some(ch) {
                1
            } else {
                0
            }
        }
        Rule::Recursive(rulesets) => rulesets
            .iter()
            .map(|subrule| non_recursive_verify_subrule(line, rules, subrule))
            .max()
            .unwrap(),
    }
}

#[aoc(day19, part1)]
fn part1((rules, lines): &Input) -> usize {
    let rule_0 = &rules[0];
    lines
        .iter()
        .filter(|line| non_recursive_verify(line, rules, rule_0) == line.len())
        .count()
}

fn recursive_verify_subrule(line: &[u8], rules: &[Rule], subrule: &RuleSet) -> Vec<usize> {
    let mut matched = Vec::new();
    let mut queue = VecDeque::new();
    // [idx, slice]
    queue.push_back((0, 0));
    while let Some((p, slice)) = queue.pop_front() {
        if slice > line.len() {
            continue;
        }
        if p == subrule.len() {
            matched.push(slice);
            continue;
        }
        for next in recursive_verify(&line[slice..], rules, &rules[subrule[p]]) {
            queue.push_back((p + 1, slice + next));
        }
    }
    matched
}

fn recursive_verify(line: &[u8], rules: &[Rule], rule: &Rule) -> Vec<usize> {
    match rule {
        Rule::Verbatim(ch) => {
            if line.first() == Some(ch) {
                vec![1]
            } else {
                vec![]
            }
        }
        Rule::Recursive(rulesets) => rulesets
            .iter()
            .flat_map(|subrule| recursive_verify_subrule(line, rules, subrule))
            .collect(),
    }
}

#[aoc(day19, part2)]
fn part2((input_rules, lines): &Input) -> usize {
    let mut rules = input_rules.clone();
    rules[8] = Rule::Recursive(vec![vec![42], vec![42, 8]]);
    rules[11] = Rule::Recursive(vec![vec![42, 31], vec![42, 11, 31]]);

    let rule_0 = &rules[0];
    lines
        .iter()
        .filter(|line| recursive_verify(line, &rules, rule_0).contains(&line.len()))
        .count()
}
