fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let input = generator(input);
    let mut cnt = 0;
    for line in input {
        if (line
            .chars()
            .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
            .count()
            >= 3)
            && (line
                .chars()
                .collect::<Vec<char>>()
                .windows(2)
                .flat_map(<&[char; 2]>::try_from)
                .any(|&[x, y]| x == y))
            && !(["ab", "cd", "pq", "xy"]
                .into_iter()
                .map(|x| line.contains(x))
                .any(|x| x))
        {
            cnt += 1
        }
    }
    cnt
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let input = generator(input);
    let mut cnt = 0;
    for line in input {
        let mut check1 = false;
        for window in line
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|x| format!("{}{}", x[0], x[1]))
        {
            if line.matches(&window).count() >= 2 {
                check1 = true;
                break;
            }
        }
        if check1
            && (line
                .chars()
                .collect::<Vec<char>>()
                .windows(3)
                .flat_map(<&[char; 3]>::try_from)
                .any(|&[x, _, y]| x == y))
        {
            cnt += 1;
        }
    }
    cnt
}
