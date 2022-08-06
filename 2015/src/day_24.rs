use itertools::Itertools;

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u64 {
    let packages: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let total_weight: u64 = packages.iter().sum();
    let weight = total_weight / 3;
    let mut good_ones = vec![];
    for i in 1..packages.len() {
        for x in packages.iter().combinations(i) {
            if x.iter().map(|&&x| x).sum::<u64>() == weight {
                good_ones.push(x);
            }
        }
        if !good_ones.is_empty() {
            break;
        }
    }
    good_ones
        .into_iter()
        .map(|x| x.into_iter().fold(1, |acc, x| acc * *x))
        .min()
        .unwrap()
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> u64 {
    let packages: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let total_weight: u64 = packages.iter().sum();
    let weight = total_weight / 4;
    let mut good_ones = vec![];
    for i in 1..packages.len() {
        for x in packages.iter().combinations(i) {
            if x.iter().map(|&&x| x).sum::<u64>() == weight {
                good_ones.push(x);
            }
        }
        if !good_ones.is_empty() {
            break;
        }
    }
    good_ones
        .into_iter()
        .map(|x| x.into_iter().fold(1, |acc, x| acc * *x))
        .min()
        .unwrap()
}
