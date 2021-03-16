#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|p| parse_pass(p))
        .filter(|p| p.is_valid())
        .count()
}
#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|p| parse_pass(p))
        .filter(|p| p.is_valid_2())
        .count()
}

fn parse_pass(pass: &str) -> Pass {
    let mut components = pass.split_ascii_whitespace();
    let mut from_to = components.next().unwrap().split('-');
    let c = components.next().unwrap().chars().next().unwrap();
    let str = components.next().unwrap();
    Pass {
        from: from_to.next().unwrap().parse().unwrap(),
        to: from_to.next().unwrap().parse().unwrap(),
        letter: c,
        pass: str,
    }
}

struct Pass<'a> {
    letter: char,
    from: usize,
    to: usize,
    pass: &'a str,
}

impl<'a> Pass<'a> {
    fn is_valid(&self) -> bool {
        let n = self
            .pass
            .chars()
            .into_iter()
            .filter(|&c| c == self.letter)
            .count();
        n >= self.from && n <= self.to
    }
    fn is_valid_2(&self) -> bool {
        let chars: Vec<_> = self.pass.chars().collect();
        (chars[self.from - 1] == self.letter) ^ (chars[self.to - 1] == self.letter)
    }
}
