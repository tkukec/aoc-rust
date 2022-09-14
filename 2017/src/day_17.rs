#[aoc(day17, part1)]
pub fn part1(input: &str) -> u32 {
    let i: usize = input.parse().unwrap();
    let mut buffer = vec![0];
    let mut ptr: usize = 1;
    for j in 1..=2017 {
        ptr = (i % j + ptr) % j + 1;
        buffer.insert(ptr, j as u32);
    }
    buffer[ptr + 1]
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u32 {
    let i: usize = input.parse().unwrap();
    let mut pos_one = 0;
    let mut ptr: usize = 1;
    for j in 1..=50000000 {
        ptr = (i % j + ptr) % j + 1;
        if ptr == 1 {
            pos_one = j;
        }
    }
    pos_one as u32
}
