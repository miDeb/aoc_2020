use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use multimap::MultiMap;
use regex::Regex;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let bags = input.lines().map(|l| parse(l));
    let mut can_contain = MultiMap::new();
    for (name, bag) in bags {
        for (_, child) in bag.children {
            can_contain.insert(child, name.clone());
        }
    }
    let mut checked = HashSet::new();
    let mut next = Vec::new();
    let mut current = Vec::new();
    next.push("shiny gold");
    while !next.is_empty() {
        std::mem::swap(&mut next, &mut current);
        for bag in &current {
            if let Some(parents) = can_contain.get_vec(*bag) {
                for parent in parents {
                    if !checked.contains(&parent) {
                        next.push(parent);
                        checked.insert(parent);
                    }
                }
            }
        }
        current.clear();
    }
    checked.len()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let bags: HashMap<String, Bag> = input.lines().map(|l| parse(l)).collect();
    bags.get("shiny gold")
        .unwrap()
        .calculate_total_children(&bags)
}

fn parse(line: &str) -> (String, Bag) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*?) bags contain").unwrap();
        static ref RE2: Regex = Regex::new(r"(?:(\d+) (.*?) bags?)").unwrap();
    }
    (
        RE.captures(line).unwrap()[1].to_owned(),
        Bag {
            children: RE2
                .captures_iter(line)
                .map(|capture| ((&capture[1]).parse().unwrap(), capture[2].to_owned()))
                .collect(),
        },
    )
}

struct Bag {
    children: Vec<(usize, String)>,
}

impl Bag {
    fn calculate_total_children(&self, bags: &HashMap<String, Bag>) -> usize {
        let mut children = 0;
        for (n, name) in &self.children {
            children += n + n * bags[name].calculate_total_children(bags);
        }
        //self.total_children = Some(children);
        children
    }
}

#[test]
fn reg() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    part1(input);
}
