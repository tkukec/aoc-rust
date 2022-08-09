use std::collections::HashMap;

#[aoc(day06, part1)]
pub fn part1(input: &str) -> String {
    let mut data = HashMap::new();
    for l in input.lines() {
        for (i, c) in l.char_indices() {
            data.entry(i).or_insert(vec![]).push(c);
        }
    }
    (0..8)
        .map(|i| {
            data[&i]
                .iter()
                .max_by(|x, y| {
                    data[&i]
                        .iter()
                        .filter(|n| n == x)
                        .count()
                        .cmp(&data[&i].iter().filter(|n| n == y).count())
                })
                .unwrap()
        })
        .collect()
}

#[aoc(day06, part2)]
pub fn part2(input: &str) -> String {
    let mut data = HashMap::new();
    for l in input.lines() {
        for (i, c) in l.char_indices() {
            data.entry(i).or_insert(vec![]).push(c);
        }
    }
    (0..8)
        .map(|i| {
            data[&i]
                .iter()
                .min_by(|x, y| {
                    data[&i]
                        .iter()
                        .filter(|n| n == x)
                        .count()
                        .cmp(&data[&i].iter().filter(|n| n == y).count())
                })
                .unwrap()
        })
        .collect()
}
