use itertools::Itertools;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|x| !x.split(' ').sorted().tuple_windows().any(|(a, b)| a == b))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|x| {
            !x.split(' ')
                .map(|x| x.chars().sorted().collect::<String>())
                .sorted()
                .tuple_windows()
                .any(|(a, b)| a == b)
        })
        .count()
}
