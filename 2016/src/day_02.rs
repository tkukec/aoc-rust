fn num_from_pos(pos: (i32, i32)) -> char {
    match pos {
        (0, 0) => '5',
        (-1, 1) => '1',
        (0, 1) => '2',
        (1, 1) => '3',
        (-1, 0) => '4',
        (1, 0) => '6',
        (-1, -1) => '7',
        (0, -1) => '8',
        (1, -1) => '9',
        _ => unreachable!(),
    }
}

#[aoc(day02, part1)]
pub fn part1(input: &str) -> String {
    let mut keycode = String::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for line in input.lines() {
        for chr in line.chars() {
            match chr {
                'U' => y = (y + 1).clamp(-1, 1),
                'D' => y = (y - 1).clamp(-1, 1),
                'R' => x = (x + 1).clamp(-1, 1),
                'L' => x = (x - 1).clamp(-1, 1),
                _ => {}
            }
        }
        keycode.push(num_from_pos((x, y)));
    }
    keycode
}

fn num_from_pos2(pos: (i32, i32)) -> char {
    match pos {
        (0, 0) => '7',
        (-1, 1) => '2',
        (0, 1) => '3',
        (1, 1) => '4',
        (-1, 0) => '6',
        (1, 0) => '8',
        (-1, -1) => 'A',
        (0, -1) => 'B',
        (1, -1) => 'C',
        (-2, 0) => '5',
        (2, 0) => '9',
        (0, -2) => 'D',
        (0, 2) => '1',
        _ => unreachable!(),
    }
}

#[aoc(day02, part2)]
pub fn part2(input: &str) -> String {
    let mut keycode = String::new();
    let mut x: i32 = -2;
    let mut y: i32 = 0;
    for line in input.lines() {
        for chr in line.chars() {
            let bound_x = 2 - y.abs();
            let bound_y = 2 - x.abs();
            match chr {
                'U' => y = (y + 1).clamp(-bound_y, bound_y),
                'D' => y = (y - 1).clamp(-bound_y, bound_y),
                'R' => x = (x + 1).clamp(-bound_x, bound_x),
                'L' => x = (x - 1).clamp(-bound_x, bound_x),
                _ => {}
            }
        }
        keycode.push(num_from_pos2((x, y)));
    }
    keycode
}
