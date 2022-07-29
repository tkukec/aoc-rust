#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut visited: Vec<(i32, i32)> = vec![(0, 0)];
    let mut x = 0;
    let mut y = 0;
    for dir in input.chars() {
        match dir {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => unreachable!(),
        }
        if !visited.contains(&(x, y)) {
            visited.push((x, y));
        }
    }
    visited.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut visited: Vec<(i32, i32)> = vec![(0, 0)];
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    for (i, dir) in input.chars().enumerate() {
        let x;
        let y;
        if i % 2 == 0 {
            x = &mut x1;
            y = &mut y1;
        } else {
            x = &mut x2;
            y = &mut y2;
        }
        match dir {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => unreachable!(),
        }
        if !visited.contains(&(*x, *y)) {
            visited.push((*x, *y));
        }
    }
    visited.len()
}
