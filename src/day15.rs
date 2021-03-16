use fxhash::FxHashMap;

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    after_steps(input, 2020)
}
#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    after_steps(input, 30000000)
}

fn after_steps(input: &str, steps: usize) -> usize {
    let mut spoken_numbers: FxHashMap<usize, usize> = FxHashMap::default();
    let mut previous = 0;
    for (idx, num) in input.split(',').enumerate() {
        let num = num.parse().unwrap();
        if idx != 0 {
            spoken_numbers.insert(previous, idx);
        }
        previous = num;
    }
    for idx in (spoken_numbers.len() + 1)..steps {
        let say = if let Some(&prev_idx) = spoken_numbers.get(&previous) {
            idx - prev_idx
        } else {
            0
        };
        spoken_numbers.insert(previous, idx);
        previous = say;
    }
    previous
}

#[test]
fn e0() {
    assert_eq!(after_steps("0,3,6", 5), 3);
    assert_eq!(after_steps("0,3,6", 10), 0);
}
#[test]
fn e1() {
    assert_eq!(part1("1,3,2"), 1);
    assert_eq!(part1("2,1,3"), 10);
    assert_eq!(part1("1,2,3"), 27);
    assert_eq!(part1("2,3,1"), 78);
    assert_eq!(part1("3,2,1"), 438);
    assert_eq!(part1("3,1,2"), 1836);
}
