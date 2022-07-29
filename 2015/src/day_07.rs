use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Op {
    RSHIFT,
    LSHIFT,
    OR,
    AND,
    NOT,
    ASSIGN,
}

fn apply_op(op: Op, x: u16, y: u16) -> u16 {
    match op {
        Op::RSHIFT => x >> y,
        Op::LSHIFT => x << y,
        Op::OR => x | y,
        Op::AND => x & y,
        _ => unreachable!(),
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Wire {
    Num(u16),
    Addr(String),
}

fn to_wire(x: &str) -> Wire {
    if let Ok(n) = x.parse::<u16>() {
        Wire::Num(n)
    } else {
        Wire::Addr(x.to_owned())
    }
}

fn generator(input: &str) -> HashMap<Wire, (Op, Wire, Option<Wire>)> {
    let mut wires: HashMap<Wire, (Op, Wire, Option<Wire>)> = HashMap::new();

    for line in input.lines() {
        let mut sides = line.split(" -> ");
        let side1: Vec<&str> = sides.next().unwrap().split(' ').collect();
        let side2: String = sides.next().unwrap().to_owned();
        match side1[..] {
            ["NOT", x] => {
                assert!(x.parse::<u16>().is_err());
                wires.insert(Wire::Addr(side2), (Op::NOT, to_wire(x), None));
            }
            [x, "RSHIFT", y] => {
                wires.insert(
                    Wire::Addr(side2),
                    (Op::RSHIFT, to_wire(x), Some(to_wire(y))),
                );
            }
            [x, "LSHIFT", y] => {
                wires.insert(
                    Wire::Addr(side2),
                    (Op::LSHIFT, to_wire(x), Some(to_wire(y))),
                );
            }
            [x, "OR", y] => {
                wires.insert(Wire::Addr(side2), (Op::OR, to_wire(x), Some(to_wire(y))));
            }
            [x, "AND", y] => {
                wires.insert(Wire::Addr(side2), (Op::AND, to_wire(x), Some(to_wire(y))));
            }
            [x] => {
                wires.insert(Wire::Addr(side2), (Op::ASSIGN, to_wire(x), None));
            }
            _ => unreachable!(),
        }
    }
    wires
}

fn solve_wire(
    wire_list: &HashMap<Wire, (Op, Wire, Option<Wire>)>,
    wire: &Wire,
    cache: &mut HashMap<Wire, u16>,
) -> u16 {
    if let Some(x) = cache.get(wire) {
        *x
    } else {
        let val = match wire {
            Wire::Num(x) => *x,
            Wire::Addr(y) => {
                let data = &wire_list[&Wire::Addr(y.to_owned())];
                match data.0 {
                    Op::ASSIGN => solve_wire(wire_list, &data.1, cache),
                    Op::NOT => !solve_wire(wire_list, &data.1, cache),
                    _ => apply_op(
                        data.0,
                        solve_wire(wire_list, &data.1, cache),
                        solve_wire(wire_list, data.2.as_ref().unwrap(), cache),
                    ),
                }
            }
        };
        cache.insert(wire.clone(), val);
        val
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u16 {
    let wires = generator(input);

    let mut cache: HashMap<Wire, u16> = HashMap::new();

    solve_wire(&wires, &Wire::Addr("a".to_owned()), &mut cache)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u16 {
    let wires = generator(input);

    let mut cache: HashMap<Wire, u16> = HashMap::new();
    cache.insert(Wire::Addr("b".to_owned()), part1(input));

    solve_wire(&wires, &Wire::Addr("a".to_owned()), &mut cache)
}
