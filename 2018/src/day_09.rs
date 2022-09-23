use std::collections::VecDeque;

#[aoc_generator(day09)]
fn generate(input: &str) -> (u32, u32) {
    let players: u32 = input.split(' ').next().unwrap().parse().unwrap();
    let last: u32 = input.split(' ').nth(6).unwrap().parse().unwrap();
    (players, last)
}

#[aoc(day09, part1)]
pub fn part1(input: &(u32, u32)) -> u32 {
    let (players, last) = *input;
    let mut scores = vec![0; players as usize];
    let mut c = VecDeque::new();
    let mut cur_player = 0;
    c.push_front(0);
    for i in 1..=last {
        cur_player = (cur_player + 1) % (players as usize);
        if i % 23 != 0 {
            let a = c.pop_back().unwrap();
            c.push_front(a);
            let a = c.pop_back().unwrap();
            c.push_front(a);

            c.push_back(i);
        } else {
            for _ in 0..7 {
                let a = c.pop_front().unwrap();
                c.push_back(a);
            }
            scores[cur_player] += i + c.pop_back().unwrap();
        }
    }
    scores.into_iter().max().unwrap()
}

#[aoc(day09, part2)]
pub fn part2(input: &(u32, u32)) -> u32 {
    part1(&(input.0, input.1 * 100))
}
