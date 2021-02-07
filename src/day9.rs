#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let values: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    for idx in 25..values.len() {
        if !sums(&values[((idx - 25) as usize)..(idx as usize)], values[idx]) {
            return values[idx];
        }
    }
    unreachable!()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    let wanted = part1(input);
    let values: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    for start_idx in 0..values.len() {
        for end_idx in (start_idx + 1)..values.len() {
            let window = &values[start_idx..=end_idx];
            let sum = window.iter().sum::<u64>();
            if sum > wanted {
                break;
            }
            if sum == wanted {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }
        }
    }
    unreachable!()
}

fn sums(slice: &[u64], val: u64) -> bool {
    for a in slice {
        for b in slice {
            if a + b == val {
                return true;
            }
        }
    }
    false
}
