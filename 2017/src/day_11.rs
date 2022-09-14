#[aoc(day11, part1)]
pub fn part1(input: &str) -> i32 {
    let mut n_s: i32 = 0;
    let mut nw_se: i32 = 0;
    let mut ne_sw: i32 = 0;
    for i in input.split(',') {
        match i {
            "n" => {
                n_s += 1;
            }
            "s" => {
                n_s -= 1;
            }
            "nw" => {
                nw_se += 1;
            }
            "se" => {
                nw_se -= 1;
            }
            "ne" => {
                ne_sw += 1;
            }
            "sw" => {
                ne_sw -= 1;
            }
            _ => unreachable!(),
        }
    }

    loop {
        if nw_se > 0 && ne_sw > 0 {
            let m = std::cmp::min(nw_se, ne_sw);
            nw_se -= m;
            ne_sw -= m;
            n_s += m;
        } else if nw_se > 0 && n_s < 0 {
            let m = std::cmp::min(nw_se, -n_s);
            nw_se -= m;
            n_s += m;
            ne_sw -= m;
        } else if n_s > 0 && nw_se < 0 {
            let m = std::cmp::min(n_s, -nw_se);
            n_s -= m;
            nw_se += m;
            ne_sw += m;
        } else if n_s > 0 && ne_sw < 0 {
            let m = std::cmp::min(n_s, -ne_sw);
            n_s -= m;
            ne_sw += m;
            nw_se += m;
        } else if ne_sw < 0 && nw_se < 0 {
            let m = std::cmp::min(-ne_sw, -nw_se);
            ne_sw += m;
            nw_se += m;
            n_s -= m;
        } else if ne_sw > 0 && n_s < 0 {
            let m = std::cmp::min(ne_sw, -n_s);
            ne_sw -= m;
            n_s += m;
            nw_se -= m;
        } else {
            break;
        }
    }
    n_s.abs() + nw_se.abs() + ne_sw.abs()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i32 {
    let mut n_s: i32 = 0;
    let mut nw_se: i32 = 0;
    let mut ne_sw: i32 = 0;
    let mut best = 0;
    for i in input.split(',') {
        match i {
            "n" => {
                n_s += 1;
            }
            "s" => {
                n_s -= 1;
            }
            "nw" => {
                nw_se += 1;
            }
            "se" => {
                nw_se -= 1;
            }
            "ne" => {
                ne_sw += 1;
            }
            "sw" => {
                ne_sw -= 1;
            }
            _ => unreachable!(),
        }
        loop {
            if nw_se > 0 && ne_sw > 0 {
                let m = std::cmp::min(nw_se, ne_sw);
                nw_se -= m;
                ne_sw -= m;
                n_s += m;
            } else if nw_se > 0 && n_s < 0 {
                let m = std::cmp::min(nw_se, -n_s);
                nw_se -= m;
                n_s += m;
                ne_sw -= m;
            } else if n_s > 0 && nw_se < 0 {
                let m = std::cmp::min(n_s, -nw_se);
                n_s -= m;
                nw_se += m;
                ne_sw += m;
            } else if n_s > 0 && ne_sw < 0 {
                let m = std::cmp::min(n_s, -ne_sw);
                n_s -= m;
                ne_sw += m;
                nw_se += m;
            } else if ne_sw < 0 && nw_se < 0 {
                let m = std::cmp::min(-ne_sw, -nw_se);
                ne_sw += m;
                nw_se += m;
                n_s -= m;
            } else if ne_sw > 0 && n_s < 0 {
                let m = std::cmp::min(ne_sw, -n_s);
                ne_sw -= m;
                n_s += m;
                nw_se -= m;
            } else {
                break;
            }
        }
        best = std::cmp::max(best, n_s.abs() + nw_se.abs() + ne_sw.abs());
    }
    best
}
