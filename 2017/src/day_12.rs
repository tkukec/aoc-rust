use std::collections::{HashMap, HashSet};

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut pipes = HashMap::new();
    for l in input.lines() {
        let a: i32 = l.split(' ').next().and_then(|x| x.parse().ok()).unwrap();
        let conns: HashSet<i32> = l
            .split(" <-> ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        pipes.insert(a, conns);
    }

    let mut seen = HashSet::new();
    let mut to_see = vec![0];
    while let Some(x) = to_see.pop() {
        seen.insert(x);
        for n in pipes[&x].iter().filter(|n| !seen.contains(n)) {
            to_see.push(*n);
        }
    }
    seen.len()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
    let mut pipes = HashMap::new();
    for l in input.lines() {
        let a: i32 = l.split(' ').next().and_then(|x| x.parse().ok()).unwrap();
        let conns: HashSet<i32> = l
            .split(" <-> ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        pipes.insert(a, conns);
    }
    let mut groups = 0;
    let mut seen_all = HashSet::new();
    while let Some(seed) = pipes.keys().find(|x| !seen_all.contains(*x)) {
        groups += 1;
        let mut seen = HashSet::new();
        let mut to_see = vec![*seed];

        while let Some(x) = to_see.pop() {
            seen.insert(x);
            seen_all.insert(x);
            for n in pipes[&x].iter().filter(|n| !seen.contains(n)) {
                to_see.push(*n);
            }
        }
    }
    groups
}
