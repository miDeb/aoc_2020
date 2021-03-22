use std::str::Split;

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    valid_messages(input, false)
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    valid_messages(input, true)
}

fn valid_messages(input: &str, part2: bool) -> usize {
    let (rules, messages) = input.split_once("\n\n").unwrap();
    let mut rules: Vec<(usize, Rule)> = rules.lines().map(|rule| parse_rule(rule, part2)).collect();
    rules.sort_by_key(|&(idx, _)| idx);
    assert_eq!(rules.last().unwrap().0, rules.len() - 1);
    let rules: Vec<Rule> = rules.into_iter().map(|(_, rule)| rule).collect();
    messages
        .lines()
        .filter(|message| rules[0].walk(message, &rules, &mut vec![]))
        .count()
}

enum Rule {
    Letter(char),
    Combination(Vec<usize>),
    Or(Vec<Rule>),
}

impl Rule {
    fn walk(&self, remaining: &str, rules: &[Rule], back_stack: &mut Vec<usize>) -> bool {
        match self {
            &Rule::Letter(l) => {
                if remaining.starts_with(l) {
                    let remaining = &remaining[1..];
                    if let Some(next) = back_stack.pop() {
                        let r = rules[next].walk(remaining, rules, back_stack);
                        back_stack.push(next);
                        r
                    } else {
                        remaining.is_empty()
                    }
                } else {
                    false
                }
            }
            Rule::Combination(list) => {
                let prev_back_stack_len = back_stack.len();
                back_stack.extend(list[1..].iter().rev());
                let r = rules[list[0]].walk(remaining, rules, back_stack);
                back_stack.truncate(prev_back_stack_len);
                assert_eq!(back_stack.len(), prev_back_stack_len);
                r
            }
            Rule::Or(options) => {
                for rule in options {
                    if rule.walk(remaining, rules, back_stack) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn parse_rule(line: &str, part2: bool) -> (usize, Rule) {
    let (idx, mut body) = line.split_once(": ").unwrap();
    if part2 {
        if idx == "8" {
            body = "42 | 42 8"
        }
        if idx == "11" {
            body = "42 31 | 42 11 31";
        }
    }
    let rule = if body.starts_with('"') {
        Rule::Letter(body.chars().nth(1).unwrap())
    } else {
        let mut tokens = body.split(' ');
        let first_numbers = parse_numbers(&mut tokens);
        let second_numbers = parse_numbers(&mut tokens);
        if second_numbers.is_empty() {
            Rule::Combination(first_numbers)
        } else {
            Rule::Or(vec![
                Rule::Combination(first_numbers),
                Rule::Combination(second_numbers),
            ])
        }
    };
    (idx.parse().unwrap(), rule)
}

fn parse_numbers(tokens: &mut Split<char>) -> Vec<usize> {
    let mut numbers = vec![];
    for token in tokens {
        if token != "|" {
            numbers.push(token.parse().unwrap());
        } else {
            break;
        }
    }
    numbers
}

#[test]
fn e0() {
    let input = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aba
aab"#;
    assert_eq!(part1(input), 2);
}
#[test]
fn e1() {
    let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
    assert_eq!(part1(input), 2);
}

#[test]
fn e2() {
    let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
29: "work around an implementation limitation"
30: "work around an implementation limitation"
32: "work around an implementation limitation"
33: "work around an implementation limitation"
34: "work around an implementation limitation"
35: "work around an implementation limitation"
36: "work around an implementation limitation"
37: "work around an implementation limitation"
38: "work around an implementation limitation"
39: "work around an implementation limitation"
40: "work around an implementation limitation"
41: "work around an implementation limitation"

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 12);
}
