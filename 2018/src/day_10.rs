use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

impl Point {
    fn upd(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
    }
}

#[aoc_generator(day10)]
fn generate(input: &str) -> Vec<Point> {
    let r = Regex::new(r"position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>").unwrap();
    input
        .lines()
        .map(|l| {
            let a = r.captures(l).unwrap();
            let x = a[1].parse().unwrap();
            let y = a[2].parse().unwrap();
            let v_x = a[3].parse().unwrap();
            let v_y = a[4].parse().unwrap();
            Point { x, y, v_x, v_y }
        })
        .collect()
}

fn display(a: &[Point]) {
    assert!(can_be_displayed(a));
    let x_min = a.iter().map(|p| p.x).min().unwrap();
    let y_min = a.iter().map(|p| p.y).min().unwrap();

    let mut grid = [[" "; 64]; 10];
    for p in a {
        grid[(p.y - y_min) as usize][(p.x - x_min) as usize] = "#";
    }
    println!("{}", grid.iter().map(|l| l.join("")).join("\n"));
}

fn can_be_displayed(a: &[Point]) -> bool {
    let x_max = a.iter().map(|p| p.x).max().unwrap();
    let y_max = a.iter().map(|p| p.y).max().unwrap();
    let x_min = a.iter().map(|p| p.x).min().unwrap();
    let y_min = a.iter().map(|p| p.y).min().unwrap();

    x_min.abs_diff(x_max) <= 64 && y_min.abs_diff(y_max) <= 10
}

#[aoc(day10, part1)]
pub fn part1(input: &[Point]) -> u32 {
    let mut input = input.to_vec();
    while !can_be_displayed(&input) {
        input.iter_mut().for_each(|p| p.upd());
    }
    display(&input);
    0
}

#[aoc(day10, part2)]
pub fn part2(input: &[Point]) -> u32 {
    let mut input = input.to_vec();
    let mut cnt = 0;
    while !can_be_displayed(&input) {
        input.iter_mut().for_each(|p| p.upd());
        cnt += 1;
    }
    cnt
}
