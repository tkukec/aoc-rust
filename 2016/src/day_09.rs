#[aoc(day09, part1)]
pub fn part1(input: &str) -> usize {
    let mut input = input.to_owned();
    let mut cnt = 0;
    while let Some(x) = input.find('(') {
        let end_x = input.find(')').unwrap();
        let mut nums = input[x + 1..end_x].split('x');
        let a = nums.next().unwrap().parse::<usize>().unwrap();
        let b = nums.next().unwrap().parse::<usize>().unwrap();

        cnt += a * b;

        input.replace_range(end_x + 1..end_x + 1 + a, "");
        input.replace_range(x..=end_x, "");
    }
    cnt + input.len()
}

// I already got burned on the lanternfish problem (2021 d6), I'm not making the same mistake twice
#[aoc(day09, part2)]
pub fn part2(input: &str) -> usize {
    let mut input = input.to_owned();
    let mut cnt = 0;
    while let Some(x) = input.find('(') {
        let end_x = input.find(')').unwrap();
        let mut nums = input[x + 1..end_x].split('x');
        let a = nums.next().unwrap().parse::<usize>().unwrap();
        let b = nums.next().unwrap().parse::<usize>().unwrap();

        // recursion because why not
        cnt += part2(&input[end_x + 1..end_x + 1 + a]) * b;

        input.replace_range(end_x + 1..end_x + 1 + a, "");
        input.replace_range(x..=end_x, "");
    }
    cnt + input.len()
}
