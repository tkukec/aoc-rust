use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    let mut data = vec![];
    for l in input.lines() {
        let name = l.split(' ').next().unwrap();
        let mut ch = HashSet::new();
        if let Some((_, c)) = l.split_once(" -> ") {
            ch = c.split(", ").collect::<HashSet<&str>>();
        }
        data.push((name, ch));
    }
    let mut a = data[0].0;
    loop {
        let maybe_parent = data.iter().find(|(_, x)| x.contains(a));
        match maybe_parent {
            Some((parent, _)) => {
                a = parent;
            }
            None => {
                return a.to_owned();
            }
        }
    }
}

fn get_weight(base: &str, data: &HashMap<&str, (i32, HashSet<&str>)>) -> i32 {
    data[base].0
        + data[base]
            .1
            .iter()
            .map(|x| get_weight(x, data))
            .sum::<i32>()
}

fn is_even(base: &str, data: &HashMap<&str, (i32, HashSet<&str>)>) -> bool {
    data[base].1.iter().map(|x| get_weight(x, data)).all_equal()
        && data[base].1.iter().map(|x| is_even(x, data)).all(|x| x)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
    let mut data = HashMap::new();
    for l in input.lines() {
        let name = l.split(' ').next().unwrap();
        let weight: i32 = l
            .split(' ')
            .nth(1)
            .unwrap()
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        let mut ch = HashSet::new();
        if let Some((_, c)) = l.split_once(" -> ") {
            ch = c.split(", ").collect::<HashSet<&str>>();
        }
        data.insert(name, (weight, ch));
    }
    let p1_sol = part1(input);
    let mut base = p1_sol.as_str();

    assert!(!is_even(base, &data));
    loop {
        let base_child_weights: Vec<(&str, i32)> = data[base]
            .1
            .iter()
            .map(|x| (*x, get_weight(x, &data)))
            .collect();
        let std_child_weight = base_child_weights
            .iter()
            .max_by(|(_, x), (_, y)| {
                base_child_weights
                    .iter()
                    .filter(|(_, n)| n == x)
                    .count()
                    .cmp(&base_child_weights.iter().filter(|(_, n)| n == y).count())
            })
            .unwrap()
            .1;

        let a: Vec<_> = base_child_weights
            .iter()
            .filter(|(_, x)| *x != std_child_weight)
            .collect();
        let unequal = *a[0];
        if is_even(unequal.0, &data) {
            return data[unequal.0].0 - (unequal.1 - std_child_weight);
        } else {
            base = unequal.0;
        }
    }
}
