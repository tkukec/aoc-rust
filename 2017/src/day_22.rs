#[derive(Debug, Clone, Copy)]
enum Dir {
    Down,
    Up,
    Right,
    Left,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Down => Dir::Left,
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
        }
    }
    fn turn_left(&mut self) {
        *self = match self {
            Dir::Down => Dir::Right,
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Left => Dir::Down,
        }
    }
    fn apply_to(&self, a: &mut (i32, i32)) {
        match self {
            Dir::Up => {
                a.1 += 1;
            }
            Dir::Down => {
                a.1 -= 1;
            }
            Dir::Right => {
                a.0 += 1;
            }
            Dir::Left => {
                a.0 -= 1;
            }
        }
    }
}

fn state_conv(a: &mut char) {
    *a = match a {
        '.' => 'W',
        'W' => '#',
        '#' => 'F',
        'F' => '.',
        _ => unreachable!(),
    }
}

fn coords_conv(a: (i32, i32), grid_size: usize) -> Option<(usize, usize)> {
    if (a.0.unsigned_abs() as usize) <= grid_size / 2
        && (a.1.unsigned_abs() as usize) <= grid_size / 2
    {
        Some((
            (grid_size as isize / 2 + a.0 as isize) as usize,
            (grid_size as isize / 2 - a.1 as isize) as usize,
        ))
    } else {
        None
    }
}

fn expand_grid(grid: &mut Vec<Vec<char>>) {
    let cur_l = grid.len();
    grid.insert(0, std::iter::repeat('.').take(cur_l).collect());
    grid.push(std::iter::repeat('.').take(cur_l).collect());
    grid.iter_mut().for_each(|x| {
        x.insert(0, '.');
        x.push('.')
    });
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut cur = (0, 0);
    let mut dir = Dir::Up;
    let mut cnt = 0;

    for _ in 0..10000 {
        let mut l = grid.len();
        let abs_cur: (usize, usize);
        if let Some(x) = coords_conv(cur, l) {
            abs_cur = x;
        } else {
            expand_grid(&mut grid);
            l += 1;
            abs_cur = coords_conv(cur, l).unwrap();
        }

        let is_infected = grid[abs_cur.1][abs_cur.0] == '#';
        if is_infected {
            dir.turn_right();
        } else {
            cnt += 1;
            dir.turn_left();
        }
        grid[abs_cur.1][abs_cur.0] = if is_infected { '.' } else { '#' };

        dir.apply_to(&mut cur);
    }
    cnt
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut cur = (0, 0);
    let mut dir = Dir::Up;
    let mut cnt = 0;

    for _ in 0..10000000 {
        let mut l = grid.len();
        let abs_cur: (usize, usize);
        if let Some(x) = coords_conv(cur, l) {
            abs_cur = x;
        } else {
            expand_grid(&mut grid);
            l += 1;
            abs_cur = coords_conv(cur, l).unwrap();
        }

        let state = grid[abs_cur.1][abs_cur.0];
        match state {
            '.' => {
                dir.turn_left();
            }
            '#' => {
                dir.turn_right();
            }
            'W' => {
                cnt += 1;
            }
            'F' => {
                dir.turn_right();
                dir.turn_right();
            }
            _ => unreachable!(),
        }
        state_conv(&mut grid[abs_cur.1][abs_cur.0]);

        dir.apply_to(&mut cur);
    }
    cnt
}
