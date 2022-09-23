fn fuel(x: u32, y: u32, i: u32) -> i32 {
    (((((x + 10) * y + i) * (x + 10)) / 100) % 10) as i32 - 5
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let i: u32 = input.parse().unwrap();
    let mut grid = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            grid[y as usize][x as usize] = fuel(y + 1, x + 1, i);
        }
    }

    let mut best = 0;
    let mut res = (0, 0);
    for y in 0..297 {
        for x in 0..297 {
            let score = grid[y][x..(x + 3)].iter().sum::<i32>()
                + grid[y + 1][x..(x + 3)].iter().sum::<i32>()
                + grid[y + 2][x..(x + 3)].iter().sum::<i32>();
            if score > best {
                best = score;
                res = (y + 1, x + 1);
            }
        }
    }
    format!("{},{}", res.0, res.1)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    let i: u32 = input.parse().unwrap();
    let mut grid = [[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            grid[y as usize][x as usize] = fuel(y + 1, x + 1, i);
        }
    }

    let mut best = 0;
    let mut res = (0, 0, 0);
    for size in 1..300 {
        for y in 0..(300 - size) {
            for x in 0..(300 - size) {
                let score = grid[y..(y + size)]
                    .iter()
                    .map(|l| l[x..(x + size)].iter().sum::<i32>())
                    .sum();
                if score > best {
                    best = score;
                    res = (y + 1, x + 1, size);
                }
            }
        }
    }
    format!("{},{},{}", res.0, res.1, res.2)
}
