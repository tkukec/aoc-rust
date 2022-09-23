use std::collections::HashSet;

#[aoc(day01, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

#[aoc(day01, part2)]
pub fn part2(input: &str) -> i32 {
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut last_sum = 0;
    loop {
        for x in input.lines() {
            last_sum += x.parse::<i32>().unwrap();
            if seen.contains(&last_sum) {
                return last_sum;
            } else {
                seen.insert(last_sum);
            }
        }
    }
}
