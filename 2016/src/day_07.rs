#[aoc(day07, part1)]
pub fn part1(input: &str) -> i32 {
    let a = input.lines().filter(|line| {
        let a = line.chars().collect::<Vec<char>>();
        let a = a
            .windows(4)
            .enumerate()
            .filter(|(_, x)| x[0] == x[3] && x[1] == x[2] && x[0] != x[1])
            .collect::<Vec<_>>();
        !a.is_empty()
            && a.iter().all(|(i, _)| {
                line[..*i].chars().filter(|c| c == &'[').count()
                    == line[..*i].chars().filter(|x| x == &']').count()
            })
    });
    a.count() as i32
}

#[aoc(day07, part2)]
pub fn part2(input: &str) -> i32 {
    let a = input.lines().filter(|line| {
        let a = line.chars().collect::<Vec<char>>();
        let a = a
            .windows(3)
            .enumerate()
            .filter(|(_, x)| x[0] == x[2] && x[0] != x[1])
            .collect::<Vec<_>>();
        !a.is_empty()
            && a.iter().any(|(i, x)| {
                line[..*i].chars().filter(|c| c == &'[').count()
                    == line[..*i].chars().filter(|x| x == &']').count()
                    && a.iter().any(|(i2, x2)| {
                        x2[0] == x[1]
                            && x2[1] == x[0]
                            && line[..*i2].chars().filter(|c| c == &'[').count()
                                > line[..*i2].chars().filter(|x| x == &']').count()
                    })
            })
    });
    a.count() as i32
}
