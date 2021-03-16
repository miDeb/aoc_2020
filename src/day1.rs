#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let input: Vec<i32> = input.split('\n').map(|s| s.parse().unwrap()).collect();
    for &a in input.iter() {
        if a >= 2020 / 2 {
            continue;
        }
        let b = 2020 - a;
        if input.contains(&b) {
            return a * b;
        }
    }
    unreachable!();
}
#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let mut vec: Vec<i32> = input.split('\n').map(|s| s.parse().unwrap()).collect();
    vec.sort_unstable();
    for &a in vec.iter() {
        for &b in vec.iter() {
            if b >= a {
                continue;
            }
            let c = 2020 - a - b;
            if c >= b {
                continue;
            }
            if vec.binary_search(&c).is_ok() {
                return a * b * c;
            }
        }
    }
    unreachable!();
}
