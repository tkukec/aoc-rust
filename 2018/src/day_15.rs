// This code is extremely bad.
// I made it work and I never want to look at it again

// Bugs my code had (ordered by debug time, ascending):
// - used the first enemy found, didn't check if there was one alphabetically before that
// - x and y axises got switched, a lot
// - an unit could be attacked / die, and if it moved after that in the same turn, it would get its health back
//   (but only in part 2, part 1 wasn't affected)
// - counted incomplete turns, but only sometimes, and fixes just broke more things
// - moving towards the wrong enemy, but only in one test case and the actual input

// at least pt2 runs in under 250ms with --release (~4s in debug)

use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Elf(i32),
    Goblin(i32),
    Wall,
    Empty,
}

impl Unit {
    fn is_free(&self) -> bool {
        *self == Unit::Empty
    }

    fn attack(&mut self) {
        match self {
            Unit::Elf(x) | Unit::Goblin(x) => *x -= 3,
            _ => panic!("Tried to attack a wall / an empty space"),
        }
    }
    fn attack_with(&mut self, power: i32) {
        match self {
            Unit::Elf(x) => *x -= 3,
            Unit::Goblin(x) => *x -= power,
            _ => panic!("Tried to attack a wall / an empty space"),
        }
    }
}

#[aoc_generator(day15)]
pub fn generate(input: &str) -> Vec<Vec<Unit>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    'E' => Unit::Elf(200),
                    'G' => Unit::Goblin(200),
                    '#' => Unit::Wall,
                    '.' => Unit::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn bfs(start: Coordinates, goals: &[Coordinates], grid: &[Vec<Unit>]) -> Option<Coordinates> {
    let mut q = VecDeque::new();
    let goals: HashSet<Coordinates> = HashSet::from_iter(goals.iter().copied());
    if goals.is_empty() {
        return None;
    }
    if goals.contains(&start) {
        return Some(start);
    }
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut parents = HashMap::new();
    q.push_back((start, 0));

    let mut ties = vec![];
    let mut max_dist = u32::MAX;
    while let Some((c, dist)) = q.pop_front() {
        if visited.contains(&c) {
            continue;
        } else {
            visited.insert(c);
            //printgrid(grid, Some(&mut visited.iter()));
        }
        let (x, y) = c;
        for (x_add, y_add) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let nx = (x as isize + x_add) as usize;
            let ny = (y as isize + y_add) as usize;

            if visited.contains(&(nx, ny)) {
                continue;
            }
            if (0..32).contains(&nx) && (0..32).contains(&ny) && (grid[ny][nx].is_free()) {
                parents.entry((nx, ny)).or_insert((x, y));
                if dist + 1 < max_dist {
                    q.push_back(((nx, ny), dist + 1));
                }
            }
            if goals.contains(&(nx, ny)) && dist <= max_dist {
                ties.push((nx, ny));
                max_dist = dist;
            }
        }
    }
    if !ties.is_empty() {
        let mut next_move = *ties.iter().min_by_key(|(x, y)| y * 32 + x).unwrap();
        while let Some(parent) = parents.get(&next_move) {
            if *parent != start {
                next_move = *parent;
            } else {
                break;
            }
        }
        assert!(
            // 0 -> stay in place
            // 1 -> move one spot
            // _ -> not allowed
            next_move.0.abs_diff(start.0) + next_move.1.abs_diff(start.1) <= 1
        );
        Some(next_move)
    } else {
        None
    }
}

type Coordinates = (usize, usize);
// pos to move to,   pos of enemy if in range
fn get_next(c: Coordinates, grid: &[Vec<Unit>]) -> (Option<Coordinates>, Option<Coordinates>) {
    let (x, y) = c;
    let is_elf = matches!(grid[y][x], Unit::Elf(_));

    let enemies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, p)| ((j, i), p)))
        .filter(|(_, x)| {
            if is_elf {
                matches!(x, Unit::Goblin(_))
            } else {
                matches!(x, Unit::Elf(_))
            }
        })
        .map(|x| x.0)
        .collect();
    let next_to_enemies: Vec<(usize, usize)> = enemies
        .iter()
        .copied()
        .flat_map(|(x, y)| {
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .map(|(nx, ny)| {
                    if grid[ny][nx].is_free() || (nx, ny) == c {
                        Some((nx, ny))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect();

    let next = bfs(c, &next_to_enemies, grid);
    let in_range = if let Some(n) = next {
        enemies
            .iter()
            .filter(|(e_x, e_y)| e_x.abs_diff(n.0) + e_y.abs_diff(n.1) == 1)
            .min_by_key(|(e_x, e_y)| match grid[*e_y][*e_x] {Unit::Elf(x) | Unit::Goblin(x) => x, _ => unreachable!()} as usize * 1000 + e_y * 32 + e_x)
    } else {
        enemies
            .iter()
            .filter(|(e_x, e_y)| e_x.abs_diff(x) + e_y.abs_diff(y) == 1)
            .min_by_key(|(e_x, e_y)| match grid[*e_y][*e_x] {Unit::Elf(x) | Unit::Goblin(x) => x, _ => unreachable!()} as usize * 1000 + e_y * 32 + e_x)
    };

    (next, in_range.copied())
}

#[aoc(day15, part1)]
pub fn part1(input: &[Vec<Unit>]) -> i32 {
    let mut grid = input.to_vec();

    let mut turns = 0;
    let mut do_next_turn = true;
    while do_next_turn {
        let mut units = grid
            .iter()
            .enumerate()
            .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, p)| ((j, i), *p)))
            .filter(|(_, x)| matches!(x, Unit::Goblin(_) | Unit::Elf(_)))
            .collect::<Vec<_>>()
            .into_iter();
        while let Some(u) = units.next() {
            do_next_turn = grid.iter().flatten().any(|x| matches!(x, Unit::Goblin(_)))
                && grid.iter().flatten().any(|x| matches!(x, Unit::Elf(_)));
            if !do_next_turn {
                break;
            }
            let (next_move, to_attack) = get_next(u.0, &grid);
            if let Some(x) = next_move {
                if x.0 != u.0 .0 || x.1 != u.0 .1 {
                    grid[u.0 .1][u.0 .0] = Unit::Empty;
                    grid[x.1][x.0] = u.1;
                }
            }
            if let Some((att_x, att_y)) = to_attack {
                grid[att_y][att_x].attack();
                if match grid[att_y][att_x] {
                    Unit::Elf(x) | Unit::Goblin(x) => x,
                    _ => 1,
                } <= 0
                {
                    grid[att_y][att_x] = Unit::Empty;
                    units = units
                        .filter(|x| x.0 != (att_x, att_y))
                        .collect::<Vec<_>>()
                        .into_iter();
                }
            }
        }
        if do_next_turn {
            turns += 1;
        }
    }
    let total_health_left: i32 = grid
        .iter()
        .flatten()
        .filter_map(|x| match x {
            Unit::Goblin(h) | Unit::Elf(h) => Some(h),
            _ => None,
        })
        .sum();

    turns * total_health_left
}

#[aoc(day15, part2)]
pub fn part2(input: &[Vec<Unit>]) -> i32 {
    let initial_elf_count = input
        .iter()
        .flatten()
        .filter(|x| matches!(x, Unit::Elf(_)))
        .count();
    'outer: for elf_power in 3.. {
        let mut grid = input.to_vec();

        let mut turns = 0;
        let mut do_next_turn = true;
        while do_next_turn {
            let mut units = grid
                .iter()
                .enumerate()
                // the compiler gets angry if i don't put move here
                .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, p)| ((j, i), *p)))
                .filter(|(_, x)| matches!(x, Unit::Goblin(_) | Unit::Elf(_)))
                .collect::<Vec<_>>()
                .into_iter();
            while let Some(((ux, uy), u)) = units.next() {
                do_next_turn = grid.iter().flatten().any(|x| matches!(x, Unit::Goblin(_)))
                    && grid.iter().flatten().any(|x| matches!(x, Unit::Elf(_)));
                if !do_next_turn {
                    break;
                }
                let (next_move, to_attack) = get_next((ux, uy), &grid);
                if let Some(x) = next_move {
                    if x.0 != ux || x.1 != uy {
                        grid[uy][ux] = Unit::Empty;
                        grid[x.1][x.0] = u;
                    }
                }
                if let Some((att_x, att_y)) = to_attack {
                    let victim = &mut grid[att_y][att_x];
                    victim.attack_with(elf_power);
                    units = units
                        .map(|x| {
                            if x.0 == (att_x, att_y) {
                                ((att_x, att_y), *victim)
                            } else {
                                x
                            }
                        })
                        .collect::<Vec<_>>()
                        .into_iter();

                    if match victim {
                        Unit::Elf(x) | Unit::Goblin(x) => *x,
                        _ => 1,
                    } <= 0
                    {
                        if matches!(victim, Unit::Elf(_)) {
                            continue 'outer;
                        }
                        *victim = Unit::Empty;
                        units = units
                            .filter(|x| x.0 != (att_x, att_y))
                            .collect::<Vec<_>>()
                            .into_iter();
                    }
                }
            }
            if do_next_turn {
                turns += 1;
            }
        }
        let total_health_left: i32 = grid
            .iter()
            .flatten()
            .filter_map(|x| match x {
                Unit::Goblin(h) | Unit::Elf(h) => Some(h),
                _ => None,
            })
            .sum();

        if grid
            .iter()
            .flatten()
            .filter(|x| matches!(x, Unit::Elf(_)))
            .count()
            == initial_elf_count
        {
            return turns * total_health_left;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_1() {
        let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(part1(&generate(input)), 27730);
        assert_eq!(part2(&generate(input)), 4988);
    }
    #[test]
    fn test2() {
        let input = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        assert_eq!(part1(&generate(input)), 36334);
        // no pt2 for some reason
    }
    #[test]
    fn test3() {
        let input = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(part1(&generate(input)), 39514);
        assert_eq!(part2(&generate(input)), 31284);
    }
    #[test]
    fn test4() {
        let input = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        assert_eq!(part1(&generate(input)), 27755);
        assert_eq!(part2(&generate(input)), 3478);
    }
    #[test]
    fn test5() {
        let input = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        assert_eq!(part1(&generate(input)), 28944);
        assert_eq!(part2(&generate(input)), 6474);
    }
    #[test]
    fn test6() {
        let input = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(part1(&generate(input)), 18740);
        assert_eq!(part2(&generate(input)), 1140);
    }
    #[test]
    fn test_my_input() {
        let input = "################################
################.#.#..##########
################.#...G##########
################...#############
######..##########.#..##########
####.G...#########.G...#########
###.........######....##########
##..#.##.....#....#....#########
#G.#GG..................##.#####
##.##..##..G........G.........##
#######......G.G...............#
#######........................#
########.G....#####..E#...E.G..#
#########G...#######...........#
#########...#########.........##
#####.......#########....G...###
###.........#########.....E..###
#...........#########.........##
#..#....G..G#########........###
#..#.........#######.........###
#G.##G......E.#####...E..E..####
##......E...............########
#.....#G.G..............E..#####
#....#####....E........###.#####
#...#########.........####.#####
#.###########......#.#####.#####
#....##########.##...###########
#....#############....##########
##.##############E....##########
##.##############..#############
##....##########################
################################";
        assert_eq!(part1(&generate(input)), 183300);
        assert_eq!(part2(&generate(input)), 40625);
    }
}
