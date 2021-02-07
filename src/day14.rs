#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut memory = vec![];
    let (mut ones_mask, mut zeros_mask) = (0u64, u64::MAX);
    for line in input.lines() {
        match &line[..2] {
            "me" => {
                let addr: usize = line["mem[".len()..].split_once(']').unwrap().0
                    .parse()
                    .unwrap();
                let mut payload: u64 = line.split_once("= ").unwrap().1.parse().unwrap();
                payload |= ones_mask;
                payload &= zeros_mask;
                if memory.len() <= addr {
                    memory.resize(addr + 1, 0);
                }
                memory[addr] = payload;
            }
            "ma" => {
                let mask = line.split_once("= ").unwrap().1;
                (ones_mask, zeros_mask) = (0, u64::MAX);
                for (idx, char) in mask.chars().rev().enumerate() {
                    match char {
                        '1' => ones_mask |= 1 << idx,
                        '0' => zeros_mask ^= 1 << idx,
                        c => assert_eq!(c, 'X'),
                    }
                }
            }
            _ => panic!("unknown instr"),
        }
    }
    memory.iter().sum()
}

#[test]
fn e1() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(part1(input), 165);
}
