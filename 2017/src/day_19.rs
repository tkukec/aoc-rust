#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Down,
    Up,
    Left,
    Right,
}

fn dir_to_coord(p: (i32, i32), dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Down => (p.0, p.1 + 1),
        Dir::Up => (p.0, p.1 - 1),
        Dir::Left => (p.0 - 1, p.1),
        Dir::Right => (p.0 + 1, p.1),
    }
}

fn get_next(p: (i32, i32), dir: &mut Dir, grid: &[Vec<char>]) -> Option<(i32, i32)> {
    let next = dir_to_coord(p, *dir);
    if grid[next.1 as usize][next.0 as usize] != ' ' {
        return Some(next);
    }
    *dir = match dir {
        Dir::Down => Dir::Left,
        Dir::Up => Dir::Right,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
    };
    let next = dir_to_coord(p, *dir);
    if grid[next.1 as usize][next.0 as usize] != ' ' {
        return Some(next);
    }
    *dir = match dir {
        Dir::Down => Dir::Up,
        Dir::Up => Dir::Down,
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
    };
    let next = dir_to_coord(p, *dir);
    if grid[next.1 as usize][next.0 as usize] != ' ' {
        return Some(next);
    }
    None
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    // (x = char num, y = line num)
    let mut path = String::new();
    let mut dir = Dir::Down;
    let mut cur = (grid[0].iter().position(|x| x == &'|').unwrap() as i32, 0);
    while let Some(new) = get_next(cur, &mut dir, &grid) {
        cur = new;
        let chr = grid[cur.1 as usize][cur.0 as usize];
        if ('A'..='Z').contains(&chr) {
            path.push(chr);
        }
    }
    path
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    // (x = char num, y = line num)
    let mut dir = Dir::Down;
    let mut cnt = 1;
    let mut cur = (grid[0].iter().position(|x| x == &'|').unwrap() as i32, 0);
    while let Some(new) = get_next(cur, &mut dir, &grid) {
        cur = new;
        cnt += 1;
    }
    cnt
}
