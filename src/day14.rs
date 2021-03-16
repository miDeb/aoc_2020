use fxhash::FxHashMap;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut memory: FxHashMap<u64, u64> = FxHashMap::default();
    let (mut ones_mask, mut zeros_mask) = (0u64, u64::MAX);
    for line in input.lines() {
        match &line[..2] {
            "me" => {
                let addr: u64 = line["mem[".len()..]
                    .split_once(']')
                    .unwrap()
                    .0
                    .parse()
                    .unwrap();
                let mut payload: u64 = line.split_once("= ").unwrap().1.parse().unwrap();
                payload |= ones_mask;
                payload &= zeros_mask;

                memory.insert(addr, payload);
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
            _ => panic!("unknown instruction"),
        }
    }
    memory.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut memory: FxHashMap<u64, u64> = FxHashMap::default();
    let mut mask = "";
    for line in input.lines() {
        match &line[..2] {
            "me" => {
                let addr: u64 = line["mem[".len()..]
                    .split_once(']')
                    .unwrap()
                    .0
                    .parse()
                    .unwrap();
                let payload: u64 = line.split_once("= ").unwrap().1.parse().unwrap();
                write_to_masked_address(&mut memory, addr, mask, payload);
            }
            "ma" => {
                mask = line.split_once("= ").unwrap().1;
            }
            _ => panic!("unknown instruction"),
        }
    }
    memory.values().sum()
}

fn write_to_masked_address(memory: &mut FxHashMap<u64, u64>, addr: u64, mask: &str, payload: u64) {
    assert!(!mask.is_empty());
    // Override with 1s on mask 1s and with 0s on mask X
    let mut first_pass_addr = addr;
    let mut x_count = 0;
    for (idx, char) in mask.chars().rev().enumerate() {
        match char {
            '1' => first_pass_addr |= 1 << idx,
            'X' => {
                first_pass_addr &= !(1 << idx);
                x_count += 1;
            }
            _ => {}
        }
    }
    let mut addresses: Vec<u64> = vec![first_pass_addr];
    memory.insert(first_pass_addr, payload);

    let mut to_push = vec![];
    for (idx, char) in mask.chars().rev().enumerate() {
        if char == 'X' {
            for &address in &addresses {
                let address = address | (1 << idx);
                memory.insert(address, payload);
                to_push.push(address);
            }
            addresses.append(&mut to_push);
        }
    }
    assert_eq!(addresses.len(), 2usize.pow(x_count));
}

#[test]
fn e1() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(part1(input), 165);
}
#[test]
fn e2() {
    let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    assert_eq!(part2(input), 208);
}
#[test]
fn e3() {
    let input = "mask = 000000000000000000000000000000000XXX
mem[8] = 4
mask = XX0000000000000000000000000000000000
mem[0] = 5";
    assert_eq!(part2(input), 52);
}
#[test]
fn e4() {
    let input = "mask = 00000000000000000000000000000000000X
mem[1] = 7
mem[0] = 3";
    assert_eq!(part2(input), 6);
}
