use std::ops::RangeInclusive;

use fxhash::FxHashSet;

#[aoc(day16, part1)]
fn part1(input: &str) -> i64 {
    let mut components = input.split("\n\n");
    let fields = parse_fields(components.next().unwrap());
    let _my_ticket = parse_ticket(components.next().unwrap().lines().nth(1).unwrap());
    let other_tickets: Vec<Vec<i64>> = components
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();
    other_tickets
        .iter()
        .map(|nearby_ticket| {
            nearby_ticket
                .iter()
                .filter(|&&value| fields.iter().all(|field| !field.is_valid(value)))
                .sum::<i64>()
        })
        .sum()
}
#[aoc(day16, part2)]
fn part2(input: &str) -> i64 {
    let mut components = input.split("\n\n");
    let fields = parse_fields(components.next().unwrap());
    let _my_ticket = parse_ticket(components.next().unwrap().lines().nth(1).unwrap());
    let other_tickets: Vec<Vec<i64>> = components
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&value| fields.iter().any(|field| field.is_valid(value)))
        })
        .collect();
    let field_order = find_field_order(&fields, &other_tickets);
    let mut acc = 1;
    for (index, field) in field_order.iter().enumerate() {
        if field.name.starts_with("departure") {
            acc *= _my_ticket[index];
        }
    }
    acc
}

fn parse_fields(input: &str) -> Vec<Field> {
    input.split('\n').map(parse_field).collect()
}

fn parse_field(line: &str) -> Field {
    let mut components = line.split(": ");
    let (name, valid_values) = (components.next().unwrap(), components.next().unwrap());
    Field {
        name,
        ranges: parse_ranges(valid_values),
    }
}

fn parse_ranges(ranges: &str) -> Vec<RangeInclusive<i64>> {
    let ranges = ranges.split(" or ");
    ranges.map(parse_range).collect()
}

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let mut from_to = range.split('-');
    from_to.next().unwrap().parse().unwrap()..=from_to.next().unwrap().parse().unwrap()
}

fn parse_ticket(line: &str) -> Vec<i64> {
    line.split(',').map(|v| v.parse().unwrap()).collect()
}

fn find_field_order<'a>(fields: &'a [Field], tickets: &[Vec<i64>]) -> Vec<&'a Field<'a>> {
    let len = fields.len();
    assert!(tickets.iter().all(|t| t.len() == len));
    let mut positioned: Vec<Vec<&'a Field<'a>>> = vec![vec![]; len];
    let mut found_rules: FxHashSet<*const Field> = FxHashSet::default();

    for rule in fields {
        for i in 0..len {
            let mut valid = true;
            for ticket in tickets {
                if !rule.is_valid(ticket[i]) {
                    valid = false;
                    break;
                }
            }
            if valid {
                positioned[i].push(rule);
            }
        }
    }
    let mut to_remove: Option<(*const Field, usize)> = None;
    'outer: loop {
        if let Some((to_remove, origin)) = to_remove {
            for (idx, rules) in positioned.iter_mut().enumerate() {
                if idx == origin {
                    continue;
                }
                let r_idx = rules.iter().position(|&r| to_remove == (r as *const _));
                if let Some(r_idx) = r_idx {
                    rules.swap_remove(r_idx);
                }
            }
        }
        for (idx, rules) in positioned.iter().enumerate() {
            if rules.len() == 1 && !found_rules.contains(&(rules[0] as *const _)) {
                to_remove = Some((rules[0], idx));
                found_rules.insert(rules[0] as *const _);
                continue 'outer;
            }
        }
        break;
    }

    assert!(positioned.iter().all(|p| p.len() == 1));

    positioned.into_iter().map(|v| v[0]).collect()
}

struct Field<'a> {
    ranges: Vec<RangeInclusive<i64>>,
    name: &'a str,
}
impl<'a> Field<'a> {
    fn is_valid(&self, value: i64) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

#[test]
fn e1() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    assert_eq!(part1(input), 71);
}
