use itertools::Itertools;

#[aoc(day02, part1)]
pub fn part1(input: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for l in input.lines() {
        let counts: Vec<usize> = l
            .chars()
            .sorted()
            .group_by(|x| *x)
            .into_iter()
            .map(|x| x.1.count())
            .collect();
        if counts.contains(&2) {
            twos += 1;
        }
        if counts.contains(&3) {
            threes += 1;
        }
    }
    println!("{twos} {threes}");
    twos * threes
}

fn count_diff(x: &str, y: &str) -> usize {
    x.chars().zip(y.chars()).filter(|(x, y)| x != y).count()
}
fn get_same(x: &str, y: &str) -> String {
    x.chars()
        .zip(y.chars())
        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
        .collect()
}

#[aoc(day02, part2)]
pub fn part2(input: &str) -> String {
    for i in input.lines() {
        for j in input.lines() {
            if count_diff(i, j) == 1 {
                return get_same(i, j);
            }
        }
    }
    unreachable!()
}
