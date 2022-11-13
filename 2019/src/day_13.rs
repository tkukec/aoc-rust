use crate::intcode::exec_at;
use std::collections::VecDeque;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();

    code.append(&mut vec![0; 1000]);
    let mut ptr = 0;
    let mut rel_off = 0;
    let mut grid = [[Tile::Empty; 200]; 200];
    let mut input = VecDeque::from([1]);
    let mut out1 = None;
    let mut out2 = None;
    while let (Some(new_ptr), output) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(output) = output {
            if out1.is_none() {
                out1 = Some(output);
            } else if out2.is_none() {
                out2 = Some(output);
            } else {
                let o1 = out1.unwrap();
                let o2 = out2.unwrap();
                let o3 = output;

                grid[(o2 + 100) as usize][(o1 + 100) as usize] = Tile::try_from(o3).unwrap();

                out1 = None;
                out2 = None;
            }
        }
    }
    grid.iter()
        .flatten()
        .filter(|x| matches!(x, Tile::Block))
        .count()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();

    code.append(&mut vec![0; 1000]);
    code[0] = 2;
    let mut ptr = 0;
    let mut rel_off = 0;
    let mut grid = [[Tile::Empty; 35]; 25];
    let mut input = VecDeque::from([0]);
    let mut out1 = None;
    let mut out2 = None;
    let mut score = 0;
    while let (Some(new_ptr), output) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(out) = output {
            if out1.is_none() {
                out1 = output;
            } else if out2.is_none() {
                out2 = output;
            } else {
                let o1 = out1.unwrap();
                let o2 = out2.unwrap();
                let o3 = out;
                if o1 == -1 && o2 == 0 {
                    score = o3;
                } else {
                    grid[(o2) as usize][(o1) as usize] = Tile::try_from(o3).unwrap();
                }

                out1 = None;
                out2 = None;
            }
        }

        if let Some(me_pos) = grid
            .iter()
            .find_map(|x| x.iter().position(|x| *x == Tile::Paddle))
        {
            if let Some(paddle_pos) = grid
                .iter()
                .find_map(|x| x.iter().position(|x| *x == Tile::Ball))
            {
                input.clear();
                input.push_back(match me_pos.cmp(&paddle_pos) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => -1,
                })
            }
        }
    }
    score
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err("Bad tile id"),
        }
    }
}
