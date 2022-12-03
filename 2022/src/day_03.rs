use itertools::Itertools;
use std::collections::HashSet;

fn prio(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - b'a' as u32 + 1
    } else {
        c as u32 - b'A' as u32 + 27
    }
}

#[aoc(day03, part1)]
pub fn part1(input: &str) -> u32 {
    let mut p = 0;
    for x in input.lines() {
        let (a, b) = x.split_at(x.len() / 2);
        let (a, b): (HashSet<char>, HashSet<char>) =
            (HashSet::from_iter(a.chars()), HashSet::from_iter(b.chars()));
        let c = a.intersection(&b);
        p += prio(*c.into_iter().next().unwrap());
    }
    p
}

#[aoc(day03, part2)]
pub fn part2(input: &str) -> u32 {
    let mut out = 0;
    for (a, b, c) in input.lines().tuples() {
        let (a, b, c): (HashSet<char>, HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(a.chars()),
            HashSet::from_iter(b.chars()),
            HashSet::from_iter(c.chars()),
        );
        let ab: HashSet<char> = a.intersection(&b).copied().collect();
        let common = *ab.intersection(&c).next().unwrap();
        out += prio(common);
    }
    out
}
