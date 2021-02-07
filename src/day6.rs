extern crate fxhash;

use fxhash::FxHashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    input.split("\n\n").map(|g| count_group(g)).sum()
}
#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    input.split("\n\n").map(|g| count_group2(g)).sum()
}

fn count_group(group: &str) -> usize {
    let mut answers = FxHashSet::default();
    for line in group.lines() {
        for char in line.chars() {
            answers.insert(char);
        }
    }
    answers.len()
}
fn count_group2(group: &str) -> usize {
    let mut lines = group.lines();
    let mut answers: FxHashSet<char> = lines.next().unwrap().chars().collect();
    for line in lines {
        let new_answers: FxHashSet<char> = line.chars().collect();
        answers.retain(|c| new_answers.contains(c));
    }
    answers.len()
}
