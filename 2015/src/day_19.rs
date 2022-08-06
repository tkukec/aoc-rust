use std::collections::HashSet;

use regex::Regex;

fn generate(input: &str) -> (Vec<(String, String)>, String) {
    let mut input = input.split("\n\n");

    let replacements: Vec<(String, String)> = input
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            let mut a = x.split(" => ");
            (a.next().unwrap().to_owned(), a.next().unwrap().to_owned())
        })
        .collect();

    (replacements, input.next().unwrap().to_owned())
}
#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let (replacements, molecule) = generate(input);
    let mut seen = HashSet::new();
    for (key, val) in replacements.iter() {
        let reg = Regex::new(key).unwrap();
        for k in reg.find_iter(&molecule) {
            let mut new = molecule.clone();
            new.replace_range(k.start()..k.end(), val);
            seen.insert(new);
        }
    }
    seen.len()
}

#[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/6909
#[aoc(day19, part2)]
pub fn part2(input: &str) -> i32 {
    let (replacements, molecule) = generate(input);
    let starts = replacements
        .clone()
        .into_iter()
        .filter(|x| x.0 == *"e".to_owned())
        .map(|(_, x)| x)
        .collect::<Vec<String>>();

    let replacements: Vec<(String, Regex)> = replacements
        .into_iter()
        .filter(|x| x.0 != *"e".to_owned())
        .map(|(x, y)| (x, Regex::new(&y).unwrap()))
        .collect();

    let mut cnt: i32 = 1;
    let mut molecule = molecule;
    loop {
        if starts.contains(&molecule) {
            break;
        }

        for (r, v) in replacements.iter() {
            if let Some(k) = v.find(&molecule.clone()) {
                molecule.replace_range(k.start()..k.end(), r);
                cnt += 1;
            }
        }
    }
    cnt
}
