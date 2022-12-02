#[aoc(day02, part1)]
pub fn part1(input: &str) -> u32 {
    //       me---
    //   # | 0 1 2
    //   --+------
    // o 0 | 3 6 0
    // p 1 | 0 3 6
    // | 2 | 6 0 3
    input
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(op, me)| {
            let (op, me) = (
                op.bytes().next().unwrap() - b'A',
                me.bytes().next().unwrap() - b'X',
            );
            let mut res = [3, 6, 0];
            res.rotate_right(op as usize);

            (res[me as usize] + me + 1) as u32
        })
        .sum()
}

#[aoc(day02, part2)]
pub fn part2(input: &str) -> u32 {
    // 0 loses to 1
    // 1 loses to 2
    // 2 loses to 0
    // (op + 2) % 3 to lose
    //
    // 0 wins against 2
    // 1 wins against 0
    // 2 wins against 1
    // (op + 1) % 3 to win
    input
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(op, me)| {
            let op = op.bytes().next().unwrap() - b'A';

            (match me {
                "X" => (op + 2) % 3,
                "Y" => 3 + op,
                "Z" => 6 + (op + 1) % 3,
                _ => unreachable!(),
            } + 1) as u32
        })
        .sum()
}
