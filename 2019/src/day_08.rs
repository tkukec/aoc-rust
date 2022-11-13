use itertools::Itertools;

#[aoc(day08, part1)]
pub fn part1(input: &str) -> usize {
    let mut input = input.chars().map(|x| x.to_digit(10).unwrap()).peekable();
    let mut layers: Vec<Vec<Vec<u8>>> = vec![];
    while input.peek().is_some() {
        let a = (0..6)
            .map(|_| (0..25).map(|_| input.next().unwrap() as u8).collect())
            .collect();
        layers.push(a);
    }
    let min = layers
        .iter()
        .min_by_key(|x| x.iter().flatten().filter(|p| **p == 0).count())
        .unwrap();
    min.iter().flatten().filter(|x| **x == 1).count()
        * min.iter().flatten().filter(|x| **x == 2).count()
}

#[aoc(day08, part2)]
pub fn part2(input: &str) -> String {
    let mut input = input.chars().map(|x| x.to_digit(10).unwrap()).peekable();
    let mut layers: Vec<Vec<Vec<u8>>> = vec![];
    while input.peek().is_some() {
        let a = (0..6)
            .map(|_| (0..25).map(|_| input.next().unwrap() as u8).collect())
            .collect();
        layers.push(a);
    }
    let mut out = layers[0].clone();

    for x in layers {
        for (p, &n) in out.iter_mut().flatten().zip(x.iter().flatten()) {
            if *p == 2 {
                *p = n;
            }
        }
    }
    String::from("\n")
        + &out
            .iter()
            .map(|l| {
                l.iter()
                    .map(|x| match x {
                        0 => ' ',
                        1 => '#',
                        _ => 'T',
                    })
                    .collect::<String>()
            })
            .join("\n")
}
