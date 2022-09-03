#[aoc(day16, part1)]
pub fn part1(input: &str) -> String {
    let mut s = input.to_string();
    while s.len() < 272 {
        let b = s.clone();
        let b = b.chars().rev();
        s.push('0');
        b.for_each(|c| s.push(if c == '1' { '0' } else { '1' }));
    }
    s = s.chars().take(272).collect();
    let mut checksum = s;
    while checksum.len() % 2 == 0 {
        let mut new_checksum = "".to_string();
        for x in checksum.chars().collect::<Vec<char>>().chunks(2) {
            if x[0] == x[1] {
                new_checksum.push('1');
            } else {
                new_checksum.push('0');
            }
        }
        checksum = new_checksum;
    }
    checksum
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> String {
    let mut s = input.to_string();
    while s.len() < 35651584 {
        let b = s.clone();
        let b = b.chars().rev();
        s.push('0');
        b.for_each(|c| s.push(if c == '1' { '0' } else { '1' }));
    }
    s = s.chars().take(35651584).collect();
    let mut checksum = s;
    while checksum.len() % 2 == 0 {
        let mut new_checksum = "".to_string();
        for x in checksum.chars().collect::<Vec<char>>().chunks(2) {
            if x[0] == x[1] {
                new_checksum.push('1');
            } else {
                new_checksum.push('0');
            }
        }
        checksum = new_checksum;
    }
    checksum
}
