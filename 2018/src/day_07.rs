use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day07)]
fn generate(input: &str) -> HashMap<char, Vec<char>> {
    // the key char can only be built if all the value chars are complete
    let mut out: HashMap<char, Vec<char>> = HashMap::new();
    for i in input.lines() {
        let first = i.chars().nth(5).unwrap();
        let second = i.chars().nth(36).unwrap();
        out.entry(second).or_default().push(first);
    }
    ('A'..='Z').for_each(|c| {
        out.entry(c).or_default();
    });
    out
}
#[aoc(day07, part1)]
pub fn part1(input: &HashMap<char, Vec<char>>) -> String {
    let mut completed = String::new();
    let mut input = input.clone();
    let mut available: Vec<char> = input
        .iter()
        .filter(|x| x.1.is_empty())
        .map(|x| *x.0)
        .collect();
    available.sort();
    available.reverse();
    while let Some(x) = available.pop() {
        completed.push(x);
        input.iter_mut().for_each(|(_, v)| v.retain(|e| *e != x));
        input
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| *k)
            .for_each(|c| available.push(c));
        available.sort();
        available.reverse();
        available.retain(|c| !completed.contains(*c));
    }

    completed
}

// < 1824
#[aoc(day07, part2)]
pub fn part2(input: &HashMap<char, Vec<char>>) -> u32 {
    let mut input = input.clone();
    let mut completed = String::new();
    let mut available: Vec<char> = input
        .iter()
        .filter(|x| x.1.is_empty())
        .map(|x| *x.0)
        .collect();
    available.sort();
    available.reverse();

    let mut time_left = HashMap::new();
    let mut time = 0;
    while !(available.is_empty() && time_left.is_empty()) {
        let mut to_add = String::new();
        time_left.iter_mut().for_each(|(_, v)| *v -= 1);
        time_left
            .clone()
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k)
            .sorted()
            .for_each(|c| {
                to_add.push(*c);
                input.iter_mut().for_each(|(_, v)| v.retain(|e| e != c));
                input
                    .iter()
                    .filter(|(_, v)| v.is_empty())
                    .map(|(k, _)| *k)
                    .for_each(|c| available.push(c));
                time_left.remove(c);
            });
        to_add.chars().sorted().for_each(|c| completed.push(c));
        available.sort();
        available.reverse();
        available.retain(|c| !completed.contains(*c));
        while time_left.len() < 5 && !available.is_empty() {
            if let Some(a) = available.pop() {
                time_left
                    .entry(a)
                    .or_insert_with(|| 61 + (a as u8 - b'A') as u32);
            }
        }

        //println!("{time} {time_left:?} {completed}");
        time += 1;
    }

    time - 1
}
