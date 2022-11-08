use std::cmp::{max, min};

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Wall,
    Flowing,
    Still,
    Empty,
}

impl Kind {
    fn is_platform(&self) -> bool {
        matches!(self, Kind::Wall | Kind::Still)
    }

    fn is_free(&self) -> bool {
        matches!(self, Kind::Empty | Kind::Flowing)
    }
}

type Point = (u32, u32);

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy)]
pub struct Line {
    x: u32,  // smallest x if it's a range
    y: u32,  // smallest y if it's a range
    a: Axis, // is the line horizontal with the x or the y axis
    other: u32,
}

impl Line {
    fn points(&self) -> Vec<Point> {
        match self.a {
            Axis::X => (self.x..=self.other).map(|x| (x, self.y)).collect(),
            Axis::Y => (self.y..=self.other).map(|y| (self.x, y)).collect(),
        }
    }
}

use once_cell::sync::Lazy;
static LINE_REG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[xy]=(\d+), ([xy])=(\d+)..(\d+)").unwrap());

impl From<&str> for Line {
    fn from(i: &str) -> Self {
        let l = LINE_REG.captures(i).expect("Bad string");
        let (a, b, c) = (
            l[1].parse().unwrap(),
            l[3].parse().unwrap(),
            l[4].parse().unwrap(),
        );
        match &l[2] {
            "x" => Line {
                x: min(b, c),
                y: a,
                a: Axis::X,
                other: max(b, c),
            },
            "y" => Line {
                x: a,
                y: min(b, c),
                a: Axis::Y,
                other: max(b, c),
            },
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day17)]
pub fn generate(input: &str) -> Vec<Line> {
    input.lines().map(Line::from).collect()
}

fn flow_down(source: Point, grid: &mut Vec<Vec<Kind>>) {
    let (x, y) = (source.0 as usize, source.1 as usize);
    grid[y][x] = Kind::Flowing;

    if grid.get(y + 1).is_none() {
        return;
    }

    if grid[y + 1][x].is_free() {
        flow_down((x as u32, (y + 1) as u32), grid);
    } else if grid[y + 1][x].is_platform() {
        // Fix borrow issues
        let bottom = grid[y + 1].clone();

        // just a QoL thing
        let a = grid.get_mut(y).unwrap();
        let left_to = a[..x]
            .iter_mut()
            .enumerate()
            .rev()
            .take_while(|(i, p)| p.is_free() && bottom[*i].is_platform())
            .map(|(_, p)| *p = Kind::Flowing)
            .count();
        let right_to = a[x..]
            .iter_mut()
            .enumerate()
            .take_while(|(i, p)| p.is_free() && bottom[*i + x].is_platform())
            .map(|(_, p)| *p = Kind::Flowing)
            .count();
        let l_bounded = a[x - left_to - 1].is_platform();
        let r_bounded = a[x + right_to].is_platform();
        if r_bounded && l_bounded {
            a[x - left_to..x + right_to]
                .iter_mut()
                .for_each(|p| *p = Kind::Still);
            flow_down((x as u32, (y - 1) as u32), grid);
        }
        if !l_bounded {
            grid[y][x - left_to - 1] = Kind::Flowing;
            flow_down(((x - left_to - 1) as u32, (y + 1) as u32), grid);
        }
        if !r_bounded {
            grid[y][x + right_to] = Kind::Flowing;
            flow_down(((x + right_to) as u32, (y + 1) as u32), grid);
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &[Line]) -> usize {
    let mut grid = vec![vec![Kind::Empty; 240]; 2100];
    for i in input {
        i.points()
            .into_iter()
            .for_each(|(x, y)| grid[y as usize][x as usize - 460] = Kind::Wall);
    }
    while !grid.last().unwrap().contains(&Kind::Wall) {
        grid.pop();
    }
    flow_down((500 - 460, 0), &mut grid);
    grid.iter()
        .flatten()
        .filter(|x| matches!(x, Kind::Flowing | Kind::Still))
        .count()
        - grid.iter().take_while(|x| !x.contains(&Kind::Wall)).count() // remove top ones too
}

// after looking at the grid in part 1 I was afraid part 2 would include water pressure
#[aoc(day17, part2)]
pub fn part2(input: &[Line]) -> usize {
    let mut grid = vec![vec![Kind::Empty; 240]; 2100];
    for i in input {
        i.points()
            .into_iter()
            .for_each(|(x, y)| grid[y as usize][x as usize - 460] = Kind::Wall);
    }
    while !grid.last().unwrap().contains(&Kind::Wall) {
        grid.pop();
    }
    flow_down((500 - 460, 0), &mut grid);
    grid.iter()
        .flatten()
        .filter(|x| matches!(x, Kind::Still))
        .count()
}
