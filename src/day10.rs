use bitvec::prelude::BitVec;

#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let jolts = collect_to_bitvec(input);
    let mut one_counter = 0;
    let mut three_counter = 1;
    let mut prev = 0;
    for (jolt, given) in jolts.iter().enumerate() {
        if !given {
            continue;
        }
        let diff = jolt - prev;
        if diff == 1 {
            one_counter += 1;
        } else if diff == 3 {
            three_counter += 1;
        }
        debug_assert!(diff <= 3, "{}, {}", jolt, diff);
        prev = jolt;
    }
    one_counter * three_counter
}
fn collect_to_bitvec(input: &str) -> BitVec {
    let mut bv = BitVec::new();
    for jolt in input.lines().map(|l| l.parse::<usize>().unwrap()) {
        if bv.len() <= jolt {
            bv.resize(jolt + 1, false);
        }
        bv.set(jolt, true);
    }
    bv
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let mut jolts = collect_to_bitvec(input);
    jolts.set(0, true);
    let mut possibilities = [0; 4];
    possibilities[1] = 1;
    for (jolt, given) in jolts.iter().enumerate().rev() {
        possibilities.copy_within(1.., 0);
        *possibilities.last_mut().unwrap() = 0;
        if !given {
            debug_assert_eq!(possibilities[0], 0);
            continue;
        }
        let this_pos = possibilities[0];
        for n in 1..=3 {
            if jolt >= n && jolts[jolt - n] {
                possibilities[n] += this_pos;
            }
        }
    }
    possibilities[0]
}

#[test]
fn e1() {
    let input = "16
10
15
5
1
11
7
19
6
12
4";
    assert_eq!(part1(input), 7 * 5);
}
#[test]
fn e2() {
    let input = "16
10
15
5
1
11
7
19
6
12
4";
    assert_eq!(part2(input), 8);
}
#[test]
fn e3() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    assert_eq!(part2(input), 19208);
}
