#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.chars().fold(0, |sum, c| match c {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => unreachable!(),
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    let mut res = 1;
    for (index, chr) in input.char_indices() {
        match chr {
            '(' => sum += 1,
            ')' => {
                if sum == 0 {
                    res += index;
                    break;
                } else {
                    sum -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    res
}
