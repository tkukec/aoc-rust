fn incr(old: &str) -> String {
    let mut b: Vec<u8> = old.bytes().collect();
    let mut add_to_next = 1;
    b.reverse();
    for i in &mut b {
        if add_to_next == 0 {
            break;
        }
        *i += add_to_next;
        if i > &mut 122 {
            *i = 97;
            add_to_next = 1;
        } else {
            add_to_next = 0;
        }
    }
    b.reverse();
    String::from_utf8(b).unwrap()
}

fn check(passw: &str) -> bool {
    let check1 = passw.bytes().collect::<Vec<u8>>()[..]
        .windows(3)
        .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2]);
    let check2 = !passw.contains('i') && !passw.contains('o') && !passw.contains('l');
    let check3 = {
        let temp = passw.bytes().collect::<Vec<u8>>();
        let win = temp[..].windows(2);
        let mut last_index = 10000;
        let mut cnt = 0;
        for (i, x) in win.enumerate() {
            if x[0] == x[1] && i != last_index + 1 {
                last_index = i;
                cnt += 1;
            }
            if cnt == 2 {
                break;
            }
        }
        cnt == 2
    };
    check1 && check2 && check3
}
fn generate_new(old: &str) -> String {
    let mut new = incr(old);
    while !check(&new) {
        new = incr(&new);
    }
    new
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    generate_new(input)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    generate_new(&generate_new(input))
}
