use std::collections::HashMap;

// No I won't write a parser for that
const N: u32 = 12523873;
const LEFT: bool = false;
const RIGHT: bool = true;
struct Res {
    to_write: bool,
    move_to: bool,
    next_state: fn(bool) -> Res,
}

fn state_a(cur: bool) -> Res {
    if !cur {
        Res {
            to_write: true,
            move_to: RIGHT,
            next_state: state_b,
        }
    } else {
        Res {
            to_write: true,
            move_to: LEFT,
            next_state: state_e,
        }
    }
}

fn state_b(cur: bool) -> Res {
    if !cur {
        Res {
            to_write: true,
            move_to: RIGHT,
            next_state: state_c,
        }
    } else {
        Res {
            to_write: true,
            move_to: RIGHT,
            next_state: state_f,
        }
    }
}

fn state_c(cur: bool) -> Res {
    if !cur {
        Res {
            to_write: true,
            move_to: LEFT,
            next_state: state_d,
        }
    } else {
        Res {
            to_write: false,
            move_to: RIGHT,
            next_state: state_b,
        }
    }
}

fn state_d(cur: bool) -> Res {
    if !cur {
        Res {
            to_write: true,
            move_to: RIGHT,
            next_state: state_e,
        }
    } else {
        Res {
            to_write: false,
            move_to: LEFT,
            next_state: state_c,
        }
    }
}

fn state_e(cur: bool) -> Res {
    if !cur {
        Res {
            to_write: true,
            move_to: LEFT,
            next_state: state_a,
        }
    } else {
        Res {
            to_write: false,
            move_to: RIGHT,
            next_state: state_d,
        }
    }
}

fn state_f(cur: bool) -> Res {
    if !cur {
        Res {
            to_write: true,
            move_to: RIGHT,
            next_state: state_a,
        }
    } else {
        Res {
            to_write: true,
            move_to: RIGHT,
            next_state: state_c,
        }
    }
}

#[aoc(day25, part1)]
pub fn part1(_input: &str) -> usize {
    let mut tape = HashMap::new();
    tape.insert(0i64, false);
    let mut cur_state: fn(bool) -> Res = state_a;
    let mut ptr = 0;
    for _ in 0..N {
        let c = tape.entry(ptr).or_insert(false);
        let r = cur_state(*c);
        *c = r.to_write;
        match r.move_to {
            RIGHT => {
                ptr += 1;
            }
            LEFT => {
                ptr -= 1;
            }
        }
        cur_state = r.next_state;
    }

    tape.values().filter(|x| **x).count()
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> String {
    "ayy".to_owned()
}
