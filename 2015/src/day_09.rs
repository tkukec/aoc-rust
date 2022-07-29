use itertools::Itertools;
use std::collections::HashMap;

fn generate(input: &str) -> (HashMap<(&str, &str), u16>, Vec<&str>) {
    let mut distances = HashMap::new();
    let mut town_list = Vec::new();
    for line in input.lines() {
        let mut line = line.split(" = ");

        let towns = line.next().unwrap().split(" to ").collect::<Vec<&str>>();
        let dist = line.next().unwrap().parse().unwrap();
        distances.insert((towns[0], towns[1]), dist);
        distances.insert((towns[1], towns[0]), dist);
        town_list.push(towns[0]);
        town_list.push(towns[1]);
    }
    town_list.sort();
    (distances, town_list.into_iter().step_by(7).collect())
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u16 {
    let (distances, towns) = generate(input);
    let len = towns.len();
    let mut cost = u16::MAX;
    for i in towns.into_iter().permutations(len).unique() {
        // only 40320 permutations, easy to brute force
        cost = std::cmp::min(cost, i.windows(2).map(|x| distances[&(x[0], x[1])]).sum());
    }
    cost
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u16 {
    let (distances, towns) = generate(input);
    let len = towns.len();
    let mut cost = u16::MIN;
    for i in towns.into_iter().permutations(len).unique() {
        // only 40320 permutations, easy to brute force
        cost = std::cmp::max(cost, i.windows(2).map(|x| distances[&(x[0], x[1])]).sum());
    }
    cost
}
