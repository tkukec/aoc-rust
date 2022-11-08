use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

type Path = Vec<PathElement>;

#[derive(Clone)]
enum PathElement {
    Dir(Dir),
    Intersection(Vec<Path>),
}

fn generate(input: &str) -> Path {
    let mut i = input.chars();
    let mut path = Path::new();
    while let Some(c) = i.next() {
        path.push(match c {
            '^' | '$' => continue,
            'N' => PathElement::Dir(Dir::N),
            'E' => PathElement::Dir(Dir::E),
            'S' => PathElement::Dir(Dir::S),
            'W' => PathElement::Dir(Dir::W),
            '(' => PathElement::Intersection({
                // Take the whole (), take_while to the first ')' doesn't work
                let mut cnt = 1;
                let a = i
                    .by_ref()
                    .take_while(|x| {
                        match x {
                            '(' => cnt += 1,
                            ')' => cnt -= 1,
                            _ => (),
                        };
                        cnt != 0
                    })
                    .collect::<String>();

                let mut cnt = 0;
                a.chars()
                    .collect::<Vec<char>>()
                    // Group the bracket expression into valid paths. Just splitting at every '|'
                    // will also split any children with intersections into invalid paths
                    .group_by(|_, b| match b {
                        '|' => cnt != 0,
                        '(' => {
                            cnt += 1;
                            true
                        }
                        ')' => {
                            cnt -= 1;
                            true
                        }
                        _ => true,
                    })
                    .into_iter()
                    .map(|x| {
                        let x: String = x.iter().collect();
                        x.strip_prefix('|').unwrap_or(&x).to_owned()
                    })
                    .map(|x| generate(&x))
                    .collect()
            }),
            other => panic!("{other}"),
        })
    }
    path
}

type Point = (i64, i64);
fn gen_adj_map(i: Path) -> (HashMap<Point, BTreeSet<Point>>, Vec<Point>) {
    let mut h: HashMap<Point, BTreeSet<Point>> = HashMap::new();
    let mut endpoints = vec![(0, 0)];
    for x in &i {
        match x {
            PathElement::Dir(d) => {
                let new = match d {
                    Dir::N => (0, -1),
                    Dir::E => (1, 0),
                    Dir::S => (0, 1),
                    Dir::W => (-1, 0),
                };
                endpoints.iter().for_each(|&e| {
                    h.entry(e).or_default().insert((e.0 + new.0, e.1 + new.1));
                    h.entry((e.0 + new.0, e.1 + new.1)).or_default().insert(e);
                });
                endpoints.iter_mut().for_each(|mut e| {
                    e.0 += new.0;
                    e.1 += new.1;
                })
            }
            PathElement::Intersection(v) => {
                let mut new_e_all = vec![];
                v.iter().for_each(|x| {
                    let x = x.to_owned();

                    let (n, mut new_e) = gen_adj_map(x);
                    for e in endpoints.iter() {
                        n.iter().for_each(|(k, v)| {
                            h.entry((e.0 + k.0, e.1 + k.1))
                                .or_default()
                                .extend(v.iter().map(|(x, y)| (x + e.0, y + e.1)))
                        })
                    }
                    new_e_all.append(&mut new_e);
                });
                let mut e2 = vec![];
                for (ex, ey) in &endpoints {
                    e2.extend(new_e_all.iter().map(|(x, y)| (x + ex, y + ey)))
                }
                endpoints = e2;
            }
        }
    }

    // IMPORTANT - Removes duplicate endpoints, there are a lot of them.
    //             The program runs out of memory if you don't dedup them
    endpoints.sort();
    endpoints.dedup();

    (h, endpoints)
}

fn print_grid(grid: &HashMap<Point, BTreeSet<Point>>) {
    let minx = *grid.keys().map(|(x, _)| x).min().unwrap();
    let maxx = *grid.keys().map(|(x, _)| x).max().unwrap();
    let miny = *grid.keys().map(|(_, y)| y).min().unwrap();
    let maxy = *grid.keys().map(|(_, y)| y).max().unwrap();
    println!("{}", "#".repeat(((maxx - minx) * 2 + 3) as usize));
    for y in miny..=maxy {
        print!("#");
        let mut line_below = String::from("#");
        for x in minx..=maxx {
            let door_e = grid.get(&(x, y)).map_or(false, |e| e.contains(&(x + 1, y)));
            let door_s = grid.get(&(x, y)).map_or(false, |e| e.contains(&(x, y + 1)));
            print!(
                "{}{}",
                if (x, y) == (0, 0) { 'X' } else { '.' },
                if door_e { '|' } else { '#' }
            );
            line_below.push(if door_s { '-' } else { '#' });
            line_below.push('#');
        }
        println!();
        println!("{line_below}");
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let i = generate(input);
    let (i, _) = gen_adj_map(i);

    print_grid(&i);

    let start = (0, 0);
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    seen.insert(start);

    loop {
        let (n, d) = q.pop_front().unwrap();

        seen.insert(n);

        if let Some(neigh) = i.get(&n) {
            let neigh = neigh.iter().filter(|x| !seen.contains(x));
            q.extend(neigh.map(|&x| (x, d + 1)))
        }

        if q.is_empty() {
            return d;
        }
    }
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let i = generate(input);
    let (i, _) = gen_adj_map(i);

    let start = (0, 0);
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    seen.insert(start);

    loop {
        let (n, d) = q.pop_front().unwrap();

        if d >= 1000 {
            return i.keys().count() - seen.len();
        }
        seen.insert(n);

        if let Some(neigh) = i.get(&n) {
            let neigh = neigh.iter().filter(|x| !seen.contains(x));
            q.extend(neigh.map(|&x| (x, d + 1)))
        }
    }
}
