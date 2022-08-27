use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
enum FloorItem {
    Generator(String),
    Chip(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Instance {
    floors: Vec<BTreeSet<FloorItem>>,
    steps: u32,
    cur_floor: u8,
}

// Converts a state into a set of pairs (GeneratorFloor, ChipFloor), because states with the same
// gen/chip positions are equivalent
//
// Part 2 takes 22 min without this optimization
//
// https://www.reddit.com/r/adventofcode/comments/5hoia9/comment/db1v1ws/
fn state_convert(x: &Instance) -> Vec<(u8, u8)> {
    let mut output = Vec::new();
    for (floor, i) in x.floors.iter().enumerate() {
        for item in i {
            if let FloorItem::Generator(name) = item {
                let chip_floor = x
                    .floors
                    .iter()
                    .position(|x| x.contains(&FloorItem::Chip(name.clone())))
                    .unwrap();
                output.push((floor as u8, chip_floor as u8));
            }
        }
    }
    output.sort();
    output
}

fn position_is_possible(position: &[&BTreeSet<FloorItem>]) -> bool {
    position.iter().all(|x| {
        x.iter().all(|it| match it {
            FloorItem::Generator(_) => true,
            // a chip can't be with another generator if it doesn't have it's own generator
            FloorItem::Chip(id) => {
                !x.iter().any(|z| matches!(z, FloorItem::Generator(_)))
                    || x.contains(&FloorItem::Generator(id.clone()))
            }
        })
    })
}

fn generate(input: &str) -> Vec<BTreeSet<FloorItem>> {
    let floors = input.lines().map(|line| {
        line.replace('.', "")
            .split(" a ")
            .skip(1)
            .map(|item| {
                let item = item.replace(" and", "").replace(',', "");
                let mut item = item.split(' ').rev();
                match item.next().unwrap() {
                    "microchip" => {
                        FloorItem::Chip(item.next().unwrap().split('-').next().unwrap().to_owned())
                    }
                    "generator" => FloorItem::Generator(item.next().unwrap().to_owned()),
                    _ => unreachable!(),
                }
            })
            .collect()
    });
    floors.collect()
}

fn solve(initial_state: Vec<BTreeSet<FloorItem>>) -> u32 {
    let mut instances = vec![Instance {
        floors: initial_state,
        steps: 0,
        cur_floor: 0,
    }];

    let mut seen = HashSet::new();
    while let Some(x) = instances.pop() {
        if seen.contains(&(state_convert(&x), x.cur_floor)) {
            continue;
        } else {
            seen.insert((state_convert(&x), x.cur_floor));
        }
        // floor is empty, elevator can't operate
        // Shouldn't be possible
        let cur = x.cur_floor as usize;

        if x.floors[0].is_empty() && x.floors[1].is_empty() && x.floors[2].is_empty() {
            return x.steps;
        }

        // moving up
        let mut moved_two = false; // if you can move two things up, don't move one thing up
        if cur != 3 {
            for i in [2, 1] {
                if !moved_two {
                    for it in x.floors[cur].clone().into_iter().combinations(i) {
                        let mut new_floors = x.floors.clone();
                        new_floors[cur].retain(|t| !it.contains(t));
                        it.into_iter().for_each(|thing| {
                            new_floors[cur + 1].insert(thing);
                        });

                        if position_is_possible(&[&new_floors[cur], &new_floors[cur + 1]]) {
                            instances.push(Instance {
                                floors: new_floors,
                                steps: x.steps + 1,
                                cur_floor: (cur + 1) as u8,
                            });
                            moved_two = true;
                        }
                    }
                }
            }
        }

        // moving down
        let mut moved_one = false; // if you moved one item down, you don't have to try two
        if cur != 0 {
            for i in [1, 2] {
                if !moved_one {
                    for it in x.floors[cur].clone().into_iter().combinations(i) {
                        let mut new_floors = x.floors.clone();
                        new_floors[cur].retain(|t| !it.contains(t));
                        if cur == 3
                            || cur == 1 && !new_floors[0].is_empty()
                            || cur == 2 && !(new_floors[1].is_empty() && new_floors[0].is_empty())
                        {
                            it.into_iter().for_each(|thing| {
                                new_floors[cur - 1].insert(thing);
                            });
                            if position_is_possible(&[&new_floors[cur], &new_floors[cur - 1]]) {
                                instances.push(Instance {
                                    floors: new_floors,
                                    steps: x.steps + 1,
                                    cur_floor: (cur - 1) as u8,
                                });
                                moved_one = true;
                            }
                        }
                    }
                }
            }
        }

        instances.sort_by(|x, y| y.steps.cmp(&x.steps));
    }
    println!("Couldn't solve. :(");
    std::process::exit(-1);
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u32 {
    let floors = generate(input);
    solve(floors)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u32 {
    let mut floors = generate(input);
    floors[0].append(&mut BTreeSet::from([
        FloorItem::Generator("elerium".to_owned()),
        FloorItem::Chip("elerium".to_owned()),
        FloorItem::Generator("dilithium".to_owned()),
        FloorItem::Chip("dilithium".to_owned()),
    ]));
    solve(floors)
}

#[test]
fn test_pt1() {
    let inp = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
    assert_eq!(part1(inp), 11);
}
