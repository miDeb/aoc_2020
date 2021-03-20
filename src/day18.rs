#[aoc(day18, part1)]
fn part1(input: &str) -> i64 {
    input.lines().map(parse_line).sum()
}

fn parse_line(input: &str) -> i64 {
    let mut tokens = Tokenizer::new(input);

    parse_expression(&mut tokens)
}

fn parse_expression(tokens: &mut Tokenizer) -> i64 {
    let mut lhs: i64 = parse_value_or_parens(tokens);
    let op = tokens.next().unwrap();
    let rhs: i64 = parse_value_or_parens(tokens);
    lhs = calc(lhs, op, rhs);
    loop {
        if matches!(tokens.peek(), None | Some(")")) {
            break;
        }
        let op = tokens.next().unwrap();
        let rhs: i64 = parse_value_or_parens(tokens);
        lhs = calc(lhs, op, rhs);
    }
    lhs
}

fn parse_value_or_parens(tokens: &mut Tokenizer) -> i64 {
    match tokens.next().unwrap() {
        "(" => {
            let val = parse_expression(tokens);
            assert_eq!(tokens.next(), Some(")"));
            val
        }
        val => val.parse().unwrap(),
    }
}

fn calc(lhs: i64, op: &str, rhs: i64) -> i64 {
    match op {
        "+" => lhs + rhs,
        "*" => lhs * rhs,
        _ => unreachable!(),
    }
}

struct Tokenizer<'a> {
    remaining: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(remaining: &'a str) -> Self {
        Self { remaining }
    }

    fn peek(&mut self) -> Option<&str> {
        let to = self.remaining.find(&[' ', ')', '('][..]);
        if let Some(to) = to {
            if to == 0 {
                let token = &self.remaining[..=0];
                Some(token)
            } else {
                let token = &self.remaining[..to];
                Some(token)
            }
        } else {
            if self.remaining.is_empty() {
                None
            } else {
                Some(self.remaining)
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let to = self.remaining.find(&[' ', ')', '('][..]);
        if let Some(to) = to {
            if to == 0 {
                let token = &self.remaining[..=0];
                self.remaining = &self.remaining[1..].trim_start();
                Some(token)
            } else {
                let token = &self.remaining[..to];
                self.remaining = &self.remaining[to..].trim_start();
                Some(token)
            }
        } else {
            if self.remaining.is_empty() {
                None
            } else {
                let token = self.remaining;
                self.remaining = "";
                Some(token)
            }
        }
    }
}

#[test]
fn e1() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(parse_line(input), 71);
}
#[test]
fn e2() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(parse_line(input), 51);
    let input = "2 * 3 + (4 * 5)";
    assert_eq!(parse_line(input), 26);
    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(parse_line(input), 437);
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(parse_line(input), 12240);
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(parse_line(input), 13632);
}
