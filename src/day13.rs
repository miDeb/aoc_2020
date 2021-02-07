use ring_algorithm::chinese_remainder_theorem;

#[aoc(day13, part1)]
fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let start: u64 = lines.next().unwrap().parse().unwrap();
    let id = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|str| match str {
            "x" => None,
            id => Some(id.parse::<u64>().unwrap()),
        })
        .min_by_key(|id| {
            let before = start % id;
            let delay = id - before;
            delay
        })
        .unwrap();
    let before = start % id;
    let delay = id - before;
    id * delay
}
#[aoc(day13, part2)]
fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let ids: Vec<(i64, i64)> = lines
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(offset, str)| match str {
            "x" => None,
            id => {
                let id = id.parse::<i64>().unwrap();
                Some((id, -(offset as i64)))
            }
        })
        .collect();
    let a = chinese_remainder_theorem(
        &ids.iter()
            .map(|&(_id, remainder)| remainder)
            .collect::<Vec<i64>>(),
        &ids.iter()
            .map(|&(id, _remainder)| id)
            .collect::<Vec<i64>>(),
    );
    a.unwrap()
}

#[test]
fn e1() {
    let input = "939
7,13,x,x,59,x,31,19";
    assert_eq!(part1(input), 295);
}

#[test]
fn e2() {
    let input = "939
7,13,x,x,59,x,31,19";
    assert_eq!(part2(input), 1068781);
}
