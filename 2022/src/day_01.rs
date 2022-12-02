#[aoc(day01, part1)]
pub fn part1(input: &str) -> u32 {
    let mut max = 0;
    for group in input.split("\n\n") {
        let sum = group.lines().map(|x| x.parse::<u32>().unwrap()).sum();
        max = std::cmp::max(sum, max);
    }
    max
}

#[aoc(day01, part2)]
pub fn part2(input: &str) -> u32 {
    let mut all = vec![];
    for group in input.split("\n\n") {
        let sum: u32 = group.lines().map(|x| x.parse::<u32>().unwrap()).sum();
        all.push(sum);
    }
    all.sort();
    all.pop().unwrap() + all.pop().unwrap() + all.pop().unwrap()
}

#[aoc(day01, part1, better)]
pub fn part1_better(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}
