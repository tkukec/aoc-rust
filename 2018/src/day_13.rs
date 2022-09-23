//   N
// W + E
//   S

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Track {
    Empty,
    NS,    // |
    WE,    // -
    NE_SW, // /
    NW_SE, // \
    Cross, // +
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
enum Turn {
    #[default]
    L,
    R,
    S,
}
impl Turn {
    fn next(&mut self) -> Self {
        let to_return = *self;
        *self = match self {
            Turn::L => Turn::S,
            Turn::S => Turn::R,
            Turn::R => Turn::L,
        };
        to_return
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cart {
    x: usize,
    y: usize,
    d: Dir,
    t: Turn,
}
impl Cart {
    fn forward(&mut self) {
        match self.d {
            Dir::N => self.y -= 1,
            Dir::S => self.y += 1,
            Dir::E => self.x += 1,
            Dir::W => self.x -= 1,
        }
    }
    fn chdir(&mut self, track: Track) {
        self.d = match self.d {
            Dir::N => match track {
                Track::NW_SE => Dir::W,
                Track::NE_SW => Dir::E,
                Track::NS => Dir::N,
                Track::Cross => match self.t.next() {
                    Turn::S => Dir::N,
                    Turn::L => Dir::W,
                    Turn::R => Dir::E,
                },
                _ => panic!("Invalid track position"),
            },
            Dir::S => match track {
                Track::NW_SE => Dir::E,
                Track::NE_SW => Dir::W,
                Track::NS => Dir::S,
                Track::Cross => match self.t.next() {
                    Turn::S => Dir::S,
                    Turn::L => Dir::E,
                    Turn::R => Dir::W,
                },
                _ => panic!("Invalid track position"),
            },
            Dir::E => match track {
                Track::NW_SE => Dir::S,
                Track::NE_SW => Dir::N,
                Track::WE => Dir::E,
                Track::Cross => match self.t.next() {
                    Turn::S => Dir::E,
                    Turn::L => Dir::N,
                    Turn::R => Dir::S,
                },
                _ => panic!("Invalid track position"),
            },
            Dir::W => match track {
                Track::NW_SE => Dir::N,
                Track::NE_SW => Dir::S,
                Track::WE => Dir::W,
                Track::Cross => match self.t.next() {
                    Turn::S => Dir::W,
                    Turn::L => Dir::S,
                    Turn::R => Dir::N,
                },
                _ => panic!("Invalid track position"),
            },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

#[aoc_generator(day13)]
pub fn generate(input: &str) -> (Vec<Vec<Track>>, Vec<Cart>) {
    let mut carts = vec![];
    let mut grid = vec![vec![Track::Empty; 150]; 150];
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            grid[i][j] = match c {
                ' ' => Track::Empty,
                '|' | 'v' | '^' => Track::NS,
                '-' | '<' | '>' => Track::WE,
                '\\' => Track::NW_SE,
                '/' => Track::NE_SW,
                '+' => Track::Cross,
                _ => unreachable!(),
            };
            if ['v', '<', '^', '>'].contains(&c) {
                carts.push(Cart {
                    x: j,
                    y: i,
                    d: match c {
                        '>' => Dir::E,
                        '<' => Dir::W,
                        '^' => Dir::N,
                        'v' => Dir::S,
                        _ => unreachable!(),
                    },
                    t: Default::default(),
                })
            }
        }
    }
    (grid, carts)
}

#[aoc(day13, part1)]
pub fn part1(input: &(Vec<Vec<Track>>, Vec<Cart>)) -> String {
    let (grid, mut carts) = input.clone();
    loop {
        // The carts with the biggest y val go to the end
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        for i in 0..carts.len() {
            let cart = carts.get_mut(i).unwrap();
            cart.forward();
            cart.chdir(grid[cart.y][cart.x]);
            let x = cart.x;
            let y = cart.y;
            if carts.iter().filter(|c| c.x == x && c.y == y).count() > 1 {
                return format!("{},{}", x, y);
            }
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &(Vec<Vec<Track>>, Vec<Cart>)) -> String {
    let (grid, mut carts) = input.clone();
    loop {
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        let mut to_wipe: Vec<Cart> = vec![];
        for i in 0..carts.len() {
            let cart = carts.get_mut(i).unwrap();
            if to_wipe.contains(cart) {
                continue;
            }
            cart.forward();
            cart.chdir(grid[cart.y][cart.x]);
            let x = cart.x;
            let y = cart.y;
            let cnt = carts.iter().filter(|c| c.x == x && c.y == y).count();
            if cnt > 1 {
                to_wipe.append(
                    &mut carts
                        .iter()
                        .copied()
                        .filter(|c| (c.x == x && c.y == y))
                        .collect(),
                );
            }

            if carts.len() - to_wipe.len() == 1 {
                let last = carts.iter().find(|x| !to_wipe.contains(x)).unwrap();
                return format!("{},{}", last.x, last.y);
            } else if carts.is_empty() {
                panic!("No carts left")
            }
        }
        carts.retain(|x| !to_wipe.contains(x));
    }
}
