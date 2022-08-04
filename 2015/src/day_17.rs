use itertools::Itertools;
#[aoc(day17, part1)]
pub fn part1(input: &str) -> i32 {
    let buckets: Vec<i32> = input.lines().filter_map(|x| x.parse().ok()).collect();
    let amount = 150;
    let mut good = 0;
    for i in 1..=buckets.len() {
        for x in buckets.iter().combinations(i) {
            if x.into_iter().sum::<i32>() == amount {
                good += 1;
            }
        }
    }
    good
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i32 {
    let buckets: Vec<i32> = input.lines().filter_map(|x| x.parse().ok()).collect();
    let amount = 150;
    let mut good = 0;
    for i in 1..=buckets.len() {
        for x in buckets.iter().combinations(i) {
            if x.into_iter().sum::<i32>() == amount {
                good += 1;
            }
        }
        if good != 0 {
            return good;
        }
    }
    unreachable!()
}
