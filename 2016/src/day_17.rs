//  x ->
// y (0, 0) (1, 0) (2, 0) (3, 0)
// | (0, 1) (1, 1) (2, 1) (3, 1)
// v (0, 2) (1, 2) (2, 2) (3, 2)
//   (0, 3) (1, 3) (2, 3) (3, 3)

fn get_next_instances(x: Instance) -> Vec<Instance> {
    let hash = md5::compute(&x.path);

    // bit shifting to get the first 4 chars of the md5 hash without getting the hex
    let u = hash[0] >> 4 >= 0xb && x.cur.1 != 0;
    let d = hash[0] << 4 >> 4 >= 0xb && x.cur.1 != 3;
    let l = hash[1] >> 4 >= 0xb && x.cur.0 != 0;
    let r = hash[1] << 4 >> 4 >= 0xb && x.cur.0 != 3;
    let mut res = vec![];
    if u {
        res.push(Instance {
            path: x.path.clone() + "U",
            cur: (x.cur.0, x.cur.1 - 1),
        });
    }
    if d {
        res.push(Instance {
            path: x.path.clone() + "D",
            cur: (x.cur.0, x.cur.1 + 1),
        });
    }
    if l {
        res.push(Instance {
            path: x.path.clone() + "L",
            cur: (x.cur.0 - 1, x.cur.1),
        });
    }
    if r {
        res.push(Instance {
            path: x.path + "R",
            cur: (x.cur.0 + 1, x.cur.1),
        });
    }
    res
}

struct Instance {
    path: String,
    cur: (u8, u8),
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let first = Instance {
        path: input.to_string(),
        cur: (0, 0),
    };
    let mut best = String::new();
    let mut instances = get_next_instances(first);
    assert!(!instances.is_empty());
    while let Some(x) = instances.pop() {
        if x.cur == (3, 3) {
            if x.path.len() < best.len() || best.is_empty() {
                best = x.path.clone();
            }
            continue;
        }
        instances.extend(get_next_instances(x));
    }
    best.strip_prefix(input).unwrap().to_string()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let first = Instance {
        path: input.to_string(),
        cur: (0, 0),
    };
    let mut best = 0;
    let mut instances = get_next_instances(first);
    assert!(!instances.is_empty());
    while let Some(x) = instances.pop() {
        if x.cur == (3, 3) {
            if x.path.len() - input.len() >= best {
                best = x.path.len() - input.len();
            }
            continue;
        }
        instances.extend(get_next_instances(x));
    }
    best
}
