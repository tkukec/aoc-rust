use std::collections::HashMap;

use md5::{compute, Digest};

fn cached_compute(input: String, cache: &mut HashMap<String, Digest>) -> Digest {
    if let Some(x) = cache.get(&input) {
        *x
    } else {
        let out = compute(input.clone());
        cache.insert(input, out);
        out
    }
}
fn contains_three(key: Digest) -> Option<u8> {
    key.iter()
        .flat_map(|x| [x >> 4, x << 4 >> 4])
        .collect::<Vec<_>>()
        .windows(3)
        .find(|x| x[0] == x[1] && x[1] == x[2])
        .map(|x| x[0])
}

fn contains_five(key: Digest, n: u8) -> bool {
    key.iter()
        .flat_map(|x| [x >> 4, x << 4 >> 4])
        .collect::<Vec<_>>()
        .windows(5)
        .any(|x| x[0] == n && x[0] == x[1] && x[1] == x[2] && x[2] == x[3] && x[3] == x[4])
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let mut key_cnt = 0;
    let mut i = 0;
    let mut cache = HashMap::new();
    while key_cnt < 64 {
        i += 1;
        if let Some(k) = contains_three(cached_compute(
            input.to_string() + &i.to_string(),
            &mut cache,
        )) {
            if ((i + 1)..=(i + 1000)).any(|x| {
                contains_five(
                    cached_compute(input.to_string() + &x.to_string(), &mut cache),
                    k,
                )
            }) {
                key_cnt += 1;
            }
        }
    }
    i
}

fn compute_2016_times(input: String, cache: &mut HashMap<String, Digest>) -> Digest {
    if let Some(x) = cache.get(&input) {
        *x
    } else {
        let mut res = format!("{:x}", compute(input.clone()));

        for _ in 0..2015 {
            res = format!("{:x}", compute(res));
        }
        let out = compute(res);
        cache.insert(input, out);
        out
    }
}
#[aoc(day14, part2)]
pub fn part2(input: &str) -> u32 {
    let mut key_cnt = 0;
    let mut i = 0;
    let mut cache = HashMap::new();
    while key_cnt < 64 {
        i += 1;
        if let Some(k) = contains_three(compute_2016_times(
            input.to_string() + &i.to_string(),
            &mut cache,
        )) {
            if ((i + 1)..=(i + 1000)).any(|x| {
                contains_five(
                    compute_2016_times(input.to_string() + &x.to_string(), &mut cache),
                    k,
                )
            }) {
                key_cnt += 1;
                println!("{key_cnt}/64");
            }
        }
    }
    i
}
