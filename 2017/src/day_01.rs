#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut out = 0;
    for x in input.chars().collect::<Vec<char>>().windows(2) {
        if x[0] == x[1] {
            out += x[0].to_digit(10).unwrap();
        }
    }
    if input.chars().last().unwrap() == input.chars().next().unwrap() {
        out += input.chars().next().unwrap().to_digit(10).unwrap()
    }
    out
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut out = 0;
    let first: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut second = first.clone();
    second.rotate_left(input.len() / 2);
    for (x, y) in std::iter::zip(first, second) {
        if x == y {
            out += x;
        }
    }
    out
}
