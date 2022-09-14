use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let i: i32 = input.parse().unwrap();
    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut y_min = 0;
    // cur dir => 0 = +x, 1 = +y, 2 = -x, 3 = -y
    let mut cur_dir = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for _ in 2..=i {
        match cur_dir {
            0 => {
                x += 1;
                if x > x_max {
                    cur_dir = (cur_dir + 1) % 4;
                    x_max += 1;
                }
            }
            1 => {
                y += 1;
                if y > y_max {
                    cur_dir = (cur_dir + 1) % 4;
                    y_max += 1;
                }
            }
            2 => {
                x -= 1;
                if x < x_min {
                    cur_dir = (cur_dir + 1) % 4;
                    x_min -= 1;
                }
            }
            3 => {
                y -= 1;
                if y < y_min {
                    cur_dir = (cur_dir + 1) % 4;
                    y_min -= 1;
                }
            }
            _ => unreachable!(),
        }
    }

    x.abs() + y.abs()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let i: i32 = input.parse().unwrap();
    let mut x_max = 1;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut y_min = 0;
    // cur dir => 0 = +x, 1 = +y, 2 = -x, 3 = -y
    let mut cur_dir = 1;
    let mut x: i32 = 1;
    let mut y: i32 = 0;
    let mut grid = HashMap::new();
    grid.insert((0, 0), 1);
    for _ in 2.. {
        let neighbors = [
            (x + 1, y),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x, y + 1),
            (x, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x - 1, y - 1),
        ];
        let val: i32 = neighbors.iter().filter_map(|n| grid.get(n)).sum();

        if val > i {
            return val;
        }

        grid.insert((x, y), val);
        match cur_dir {
            0 => {
                x += 1;
                if x > x_max {
                    cur_dir = (cur_dir + 1) % 4;
                    x_max += 1;
                }
            }
            1 => {
                y += 1;
                if y > y_max {
                    cur_dir = (cur_dir + 1) % 4;
                    y_max += 1;
                }
            }
            2 => {
                x -= 1;
                if x < x_min {
                    cur_dir = (cur_dir + 1) % 4;
                    x_min -= 1;
                }
            }
            3 => {
                y -= 1;
                if y < y_min {
                    cur_dir = (cur_dir + 1) % 4;
                    y_min -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    unreachable!()
}
