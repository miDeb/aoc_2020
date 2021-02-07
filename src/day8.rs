#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let mut vm = Vm::new(input.lines().map(|line| parse(line)).collect());
    vm.run_until_loop()
}
#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let mut vm = Vm::new(input.lines().map(|line| parse(line)).collect());
    for i in 0..vm.instructions.len() {
        let prev_val = vm.instructions[i].op;
        vm.instructions[i].op = match prev_val {
            Op::Noop(val) => Op::Jmp(val),
            Op::Jmp(val) => Op::Noop(val),
            v => v,
        };
        if !vm.has_loop() {
            return vm.acc;
        }
        vm.instructions[i].op = prev_val;
        vm.reset();
    }
    panic!()
}

struct Instruction {
    visited: bool,
    op: Op,
}

#[derive(Clone, Copy)]
enum Op {
    Noop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse(line: &str) -> Instruction {
    let arg = line[4..].parse().unwrap();
    let op = match &line[..=2] {
        "acc" => Op::Acc(arg),
        "jmp" => Op::Jmp(arg),
        "nop" => Op::Noop(arg),
        _ => unreachable!(),
    };
    Instruction { visited: false, op }
}

struct Vm {
    instructions: Vec<Instruction>,
    acc: i32,
    ip: usize,
}

impl Vm {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            acc: 0,
            ip: 0,
        }
    }

    fn exec_at_ip(&mut self) {
        let instruction = &mut self.instructions[self.ip];
        let mut inc = 1;
        match instruction.op {
            Op::Noop(_) => {}
            Op::Acc(arg) => {
                self.acc += arg;
            }
            Op::Jmp(arg) => inc = arg,
        }
        self.ip = (self.ip as i32 + inc) as usize;
        instruction.visited = true;
    }

    fn run_until_loop(&mut self) -> i32 {
        while !self.instructions[self.ip].visited {
            self.exec_at_ip();
        }
        self.acc
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
        for instr in &mut self.instructions {
            instr.visited = false;
        }
    }

    fn has_loop(&mut self) -> bool {
        loop {
            if self.ip >= self.instructions.len() {
                break false;
            }
            if self.instructions[self.ip].visited {
                break true;
            }
            self.exec_at_ip();
        }
    }
}
