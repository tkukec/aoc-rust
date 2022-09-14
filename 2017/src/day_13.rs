use std::collections::HashMap;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let a: HashMap<i32, i32> = input
        .lines()
        .map(|s| {
            let mut it = s.split(": ").map(|x| x.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    let mut total = 0;
    for i in 0..=(*a.keys().max().unwrap()) {
        if let Some(range) = a.get(&i) {
            if i % (range * 2 - 2) == 0 {
                total += range * i;
            }
        }
    }
    total
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i32 {
    let a: HashMap<i32, i32> = input
        .lines()
        .map(|s| {
            let mut it = s.split(": ").map(|x| x.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    let largest = *a.keys().max().unwrap();
    'outer: for d in 0.. {
        for i in 0..=largest {
            if let Some(range) = a.get(&i) {
                if (d + i) % (range * 2 - 2) == 0 {
                    continue 'outer;
                }
            }
        }
        return d;
    }
    unreachable!()
}
