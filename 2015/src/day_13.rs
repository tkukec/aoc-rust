use itertools::Itertools;
use std::collections::HashMap;

fn generate(input: &str) -> (HashMap<(&str, &str), i32>, Vec<&str>) {
    let mut out = HashMap::new();
    let mut names = Vec::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let name1 = line[0];
        let mut name2 = *line.last().unwrap();
        name2 = &name2[..name2.len() - 1];
        let score: i32 = match line[2] {
            "gain" => line[3].parse().unwrap(),
            "lose" => -line[3].parse::<i32>().unwrap(),
            _ => unreachable!(),
        };
        *out.entry((name1, name2)).or_insert(0) += score;
        *out.entry((name2, name1)).or_insert(0) += score;
        names.push(name1);
    }
    let len = names.len();
    (
        out,
        names
            .into_iter()
            .step_by((len as f64).sqrt() as usize)
            .collect(),
    )
}

fn score<'a>(data: &HashMap<(&'a str, &'a str), i32>, order: &'a [&str]) -> i32 {
    [order, &order[..1]]
        .concat()
        .windows(2)
        .map(|n| data[&(n[0], n[1])])
        .sum()
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let (data, names) = generate(input);
    let len = names.len();
    names
        .into_iter()
        .permutations(len)
        .map(|x| score(&data, &x))
        .max()
        .unwrap()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i32 {
    let (mut data, mut names) = generate(input);
    for person in &names {
        data.insert(("me", person), 0);
        data.insert((person, "me"), 0);
    }
    names.push("me");

    let len = names.len();
    names
        .into_iter()
        .permutations(len)
        .map(|x| score(&data, &x))
        .max()
        .unwrap()
}
