use std::fmt::Write;

#[derive(Debug)]
enum Command {
    Rect(i32, i32),   // x, y
    RotRow(i32, i32), // row_num, shift_amount
    RotCol(i32, i32), // col_num, shift_amount
}

fn generate(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            match it.next().unwrap() {
                "rotate" => {
                    let orientation = it.next().unwrap();
                    let num = it.next().unwrap()[2..].parse().unwrap();
                    let amount = it.nth(1).unwrap().parse().unwrap();
                    match orientation {
                        "row" => Command::RotRow(num, amount),
                        "column" => Command::RotCol(num, amount),
                        _ => unreachable!(),
                    }
                }
                "rect" => {
                    let mut a = it.next().unwrap().split('x');
                    Command::Rect(
                        // unwrap the unwrapped unwrap
                        a.next().unwrap().parse().unwrap(),
                        a.next().unwrap().parse().unwrap(),
                    )
                }
                _ => unreachable!(),
            }
        })
        .collect()
}
#[aoc(day08, part1)]
pub fn part1(input: &str) -> i32 {
    let data = generate(input);
    let mut grid = [[false; 50]; 6];
    for x in &data {
        match x {
            Command::Rect(x, y) => {
                (0..*y).for_each(|y1| (0..*x).for_each(|x1| grid[y1 as usize][x1 as usize] = true))
            }
            Command::RotRow(num, shift) => {
                grid[*num as usize].rotate_right(*shift as usize);
            }
            Command::RotCol(num, shift) => {
                for _ in 0..*shift {
                    for x in 1..grid.len() {
                        // XOR shift because idk how to mem::swap things from the same slice
                        grid[0][*num as usize] ^= grid[x][*num as usize];
                        grid[x][*num as usize] ^= grid[0][*num as usize];
                        grid[0][*num as usize] ^= grid[x][*num as usize];
                    }
                }
            }
        }
    }
    grid.into_iter().flatten().filter(|x| *x).count() as i32
}

#[aoc(day08, part2)]
pub fn part2(input: &str) -> String {
    let data = generate(input);
    let mut grid = [[false; 50]; 6];
    for x in &data {
        match x {
            Command::Rect(x, y) => {
                (0..*y).for_each(|y1| (0..*x).for_each(|x1| grid[y1 as usize][x1 as usize] = true))
            }
            Command::RotRow(num, shift) => {
                grid[*num as usize].rotate_right(*shift as usize);
            }
            Command::RotCol(num, shift) => {
                for _ in 0..*shift {
                    for x in 1..grid.len() {
                        grid[0][*num as usize] ^= grid[x][*num as usize];
                        grid[x][*num as usize] ^= grid[0][*num as usize];
                        grid[0][*num as usize] ^= grid[x][*num as usize];
                    }
                }
            }
        }
    }
    let mut res = String::new();
    for line in grid {
        write!(
            &mut res,
            "\n{}",
            line.map(|x| if x { "█" } else { " " }).join("")
        )
        .unwrap();
    }
    res
}
