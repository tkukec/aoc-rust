use std::cmp::Ordering;
#[aoc_generator(day24)]
pub fn generate(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|x| {
            let a = x.split_once('/').unwrap();
            (a.0.parse().unwrap(), a.1.parse().unwrap())
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[(i32, i32)]) -> i32 {
    let mut instances = input
        .iter()
        .filter(|x| x.0 == 0 || x.1 == 0)
        .map(|c| vec![(0, 0), *c])
        .collect::<Vec<_>>();

    let mut best = 0;

    while let Some(x) = instances.pop() {
        let score = x.iter().map(|x| x.0 + x.1).sum();
        best = std::cmp::max(best, score);
        let last_i = x.len() - 1;
        let a = x[last_i];
        let b = x[last_i - 1];

        // calculate what the last port is, so i don't have to store it
        let last_num = if a.0 == b.0 || a.0 == b.1 { a.1 } else { a.0 };

        for l in input
            .iter()
            .filter(|x| x.0 == last_num || x.1 == last_num)
            .filter(|n| !x.contains(n))
        {
            let mut new_vec = x.clone();
            new_vec.push(*l);
            instances.push(new_vec);
        }
    }
    best
}

#[aoc(day24, part2)]
pub fn part2(input: &[(i32, i32)]) -> i32 {
    let mut instances = input
        .iter()
        .filter(|x| x.0 == 0 || x.1 == 0)
        .map(|c| vec![(0, 0), *c])
        .collect::<Vec<_>>();
    let mut best = 0;
    let mut longest = 0;
    while let Some(x) = instances.pop() {
        let score = x.iter().map(|x| x.0 + x.1).sum();
        match x.len().cmp(&longest) {
            Ordering::Greater => {
                longest = x.len();
                best = score;
            }
            Ordering::Equal => {
                best = std::cmp::max(best, score);
            }
            Ordering::Less => {}
        }
        let last_i = x.len() - 1;
        let a = x[last_i];
        let b = x[last_i - 1];

        let last_num = if a.0 == b.0 || a.0 == b.1 { a.1 } else { a.0 };
        for l in input
            .iter()
            .filter(|x| x.0 == last_num || x.1 == last_num)
            .filter(|n| !x.contains(n))
        {
            let mut new_vec = x.clone();
            new_vec.push(*l);
            instances.push(new_vec);
        }
    }
    best
}
