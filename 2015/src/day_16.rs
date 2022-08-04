use std::collections::HashMap;

use regex::Regex;

fn generate_values() -> HashMap<&'static str, i32> {
    let known = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";
    let mut values = HashMap::new();
    known.lines().for_each(|x| {
        values.insert(
            x.split(": ").collect::<Vec<&str>>()[0],
            x.split(": ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap(),
        );
    });
    values
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> String {
    let values = generate_values();
    let reg = Regex::new("(\\w+): (\\d+)").unwrap();
    let mut possible: Vec<&str> = input.lines().collect();
    possible = possible
        .into_iter()
        .filter(|line| {
            reg.captures_iter(line)
                .all(|x| x[2].parse::<i32>().unwrap() == values[&x[1]])
        })
        .collect();

    possible.join("\r\n")
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> String {
    let values = generate_values();
    let reg = Regex::new("(\\w+): (\\d+)").unwrap();
    let mut possible: Vec<&str> = input.lines().collect();
    possible = possible
        .into_iter()
        .filter(|line| {
            reg.captures_iter(line).all(|x| match &x[1] {
                "cats" | "trees" => x[2].parse::<i32>().unwrap() > values[&x[1]],
                "pomeranians" | "goldfish" => x[2].parse::<i32>().unwrap() < values[&x[1]],
                _ => x[2].parse::<i32>().unwrap() == values[&x[1]],
            })
        })
        .collect();

    possible.join("\r\n")
}
