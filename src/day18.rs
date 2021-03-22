use std::mem;

#[aoc(day18, part1)]
fn part1(input: &str) -> i64 {
    input.lines().map(parse_line::<{ Priorities::None }>).sum()
}
#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line::<{ Priorities::Inverse }>)
        .sum()
}

fn parse_line<const P: Priorities>(input: &str) -> i64 {
    let mut tokens = Tokenizer::new(input);

    parse_expression::<P>(&mut tokens).eval()
}

fn parse_expression<const P: Priorities>(tokens: &mut Tokenizer) -> AstNode<P> {
    let mut lhs = parse_value_or_parens(tokens);
    let op = tokens.next().unwrap();
    let rhs = Box::new(parse_value_or_parens(tokens));
    lhs = lhs.append(op, rhs);
    loop {
        if matches!(tokens.peek(), None | Some(")")) {
            break;
        }
        let op = tokens.next().unwrap();
        let rhs = Box::new(parse_value_or_parens(tokens));
        lhs = lhs.append(op, rhs);
    }
    lhs
}

fn parse_value_or_parens<const P: Priorities>(tokens: &mut Tokenizer) -> AstNode<P> {
    match tokens.next().unwrap() {
        "(" => {
            let val = parse_expression(tokens);
            assert_eq!(tokens.next(), Some(")"));
            AstNode::Paren(Box::new(val))
        }
        val => AstNode::Number(val.parse().unwrap()),
    }
}

#[derive(PartialEq, Eq)]
enum Priorities {
    None,
    Inverse,
}

enum AstNode<const P: Priorities> {
    Paren(Box<AstNode<P>>),
    Plus(Box<AstNode<P>>, Box<AstNode<P>>),
    Times(Box<AstNode<P>>, Box<AstNode<P>>),
    Number(i64),
}

impl<const P: Priorities> AstNode<P> {
    fn priority(&self) -> u8 {
        match P {
            Priorities::None => 0,
            Priorities::Inverse => match self {
                AstNode::Number(_) => 4,
                AstNode::Paren(_) => 3,
                AstNode::Plus(_, _) => 2,
                AstNode::Times(_, _) => 1,
            },
        }
    }

    fn op_priority(op: &str) -> u8 {
        match P {
            Priorities::None => 0,
            Priorities::Inverse => match op {
                "+" => 2,
                "*" => 1,
                _ => unreachable!(),
            },
        }
    }

    fn append(mut self, op: &str, rhs: Box<AstNode<P>>) -> AstNode<P> {
        if self.priority() >= AstNode::<P>::op_priority(op) {
            AstNode::from_op(Box::new(self), op, rhs)
        } else if let AstNode::Times(_, prev_rhs) = &mut self {
            let mut tmp = AstNode::<P>::Number(-1);
            mem::swap(&mut tmp, prev_rhs);
            *prev_rhs = Box::new(AstNode::from_op(Box::new(tmp), op, rhs));
            self
        } else {
            unreachable!();
        }
    }

    fn from_op(lhs: Box<AstNode<P>>, op: &str, rhs: Box<AstNode<P>>) -> AstNode<P> {
        match op {
            "+" => AstNode::Plus(lhs, rhs),
            "*" => AstNode::Times(lhs, rhs),
            _ => unreachable!(),
        }
    }

    fn eval(&self) -> i64 {
        match self {
            AstNode::Paren(node) => node.eval(),
            AstNode::Plus(lhs, rhs) => lhs.eval() + rhs.eval(),
            AstNode::Times(lhs, rhs) => lhs.eval() * rhs.eval(),
            AstNode::Number(num) => *num,
        }
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
        } else if self.remaining.is_empty() {
            None
        } else {
            Some(self.remaining)
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
        } else if self.remaining.is_empty() {
            None
        } else {
            let token = self.remaining;
            self.remaining = "";
            Some(token)
        }
    }
}

#[test]
fn e1() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(parse_line::<{ Priorities::None }>(input), 71);
}
#[test]
fn e2() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(parse_line::<{ Priorities::None }>(input), 51);
    let input = "2 * 3 + (4 * 5)";
    assert_eq!(parse_line::<{ Priorities::None }>(input), 26);
    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(parse_line::<{ Priorities::None }>(input), 437);
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(parse_line::<{ Priorities::None }>(input), 12240);
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(parse_line::<{ Priorities::None }>(input), 13632);
}

#[test]
fn e3() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(parse_line::<{ Priorities::Inverse }>(input), 231);
    let input = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(parse_line::<{ Priorities::Inverse }>(input), 51);
    let input = "2 * 3 + (4 * 5)";
    assert_eq!(parse_line::<{ Priorities::Inverse }>(input), 46);
    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(parse_line::<{ Priorities::Inverse }>(input), 1445);
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(parse_line::<{ Priorities::Inverse }>(input), 669060);
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(parse_line::<{ Priorities::Inverse }>(input), 23340);
}
