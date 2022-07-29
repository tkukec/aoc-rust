#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut a = [[false; 1000]; 1000];
    enum Command {
        Off,
        On,
        Toggle,
    }
    fn change(
        grid: &mut [[bool; 1000]; 1000],
        command: Command,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
    ) {
        for x in grid.iter_mut().take(x1 + 1).skip(x0) {
            for y in x.iter_mut().take(y1 + 1).skip(y0) {
                *y = match command {
                    Command::Off => false,
                    Command::On => true,
                    Command::Toggle => !*y,
                }
            }
        }
    }

    for line in input.lines() {
        let mut coords = line.split(' ').filter(|x| x.contains(',')).map(|x| {
            x.split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        });
        let cor1 = coords.next().unwrap();
        let cor2 = coords.next().unwrap();
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["turn", "off", ..] => change(&mut a, Command::Off, cor1[0], cor1[1], cor2[0], cor2[1]),
            ["turn", "on", ..] => change(&mut a, Command::On, cor1[0], cor1[1], cor2[0], cor2[1]),
            ["toggle", ..] => change(&mut a, Command::Toggle, cor1[0], cor1[1], cor2[0], cor2[1]),
            _ => unreachable!(),
        }
    }

    a.into_iter().flatten().into_iter().filter(|x| *x).count()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let mut a = [[0; 1000]; 1000];
    enum Command {
        Off,
        On,
        Toggle,
    }
    fn change(
        grid: &mut [[u8; 1000]; 1000],
        command: Command,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
    ) {
        for x in grid.iter_mut().take(x1 + 1).skip(x0) {
            for y in x.iter_mut().take(y1 + 1).skip(y0) {
                *y = match command {
                    Command::Off => y.saturating_sub(1), // limit light level to 0, negatives aren't allowed
                    Command::On => *y + 1,
                    Command::Toggle => *y + 2,
                }
            }
        }
    }

    for line in input.lines() {
        // get the coordinates from the instruction
        let mut coords = line.split(' ').filter(|x| x.contains(',')).map(|x| {
            x.split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        });
        let cor1 = coords.next().unwrap();
        let cor2 = coords.next().unwrap();
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["turn", "off", ..] => change(&mut a, Command::Off, cor1[0], cor1[1], cor2[0], cor2[1]),
            ["turn", "on", ..] => change(&mut a, Command::On, cor1[0], cor1[1], cor2[0], cor2[1]),
            ["toggle", ..] => change(&mut a, Command::Toggle, cor1[0], cor1[1], cor2[0], cor2[1]),
            _ => unreachable!(),
        }
    }

    a.into_iter().flatten().map(|x| x as u32).sum()
}
