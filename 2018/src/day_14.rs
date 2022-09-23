#[aoc(day14, part1)]
pub fn part1(input: &str) -> String {
    let i: usize = input.parse().unwrap();
    let mut scoreboard = vec![3u8, 7];
    let mut pos1 = 0;
    let mut pos2 = 1;
    let mut l = 2;
    while l < (i + 10) {
        let v1 = scoreboard[pos1];
        let v2 = scoreboard[pos2];
        let new_v = v1 + v2;
        if new_v >= 10 {
            scoreboard.push(new_v / 10);
            scoreboard.push(new_v % 10);
            l += 2;
        } else {
            scoreboard.push(new_v);
            l += 1;
        }
        pos1 = (pos1 + 1 + v1 as usize) % l;
        pos2 = (pos2 + 1 + v2 as usize) % l;
    }
    scoreboard[i..(i + 10)]
        .iter()
        .map(|x| x.to_string())
        .collect()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let ilen = input.len();
    let i = &input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()[..];
    let mut scoreboard = vec![3u8, 7];
    let mut pos1 = 0;
    let mut pos2 = 1;
    let mut l = 2;
    loop {
        let v1 = scoreboard[pos1];
        let v2 = scoreboard[pos2];
        let new_v = v1 + v2;
        if new_v >= 10 {
            scoreboard.push(new_v / 10);
            scoreboard.push(new_v % 10);
            l += 2;
        } else {
            scoreboard.push(new_v);
            l += 1;
        }
        if l > (ilen + 1) {
            // check the 2 ilen length windows in the back, the sublist might not include
            // the last character if 2 were added
            if let Some(x) = scoreboard[(l - ilen - 1)..l]
                .windows(ilen)
                .position(|x| x == i)
            {
                return l - ilen - 1 + x;
            }
        }
        pos1 = (pos1 + 1 + v1 as usize) % l;
        pos2 = (pos2 + 1 + v2 as usize) % l;
    }
}
