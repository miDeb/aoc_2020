use bitvec::prelude::*;

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(|line| get_id(line)).max().unwrap()
}
#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let mut seats = bitarr![0; 128*8];
    for seat in input.lines().map(|line| get_id(line)) {
        seats.set(seat, true)
    }
    for r in 0..128 {
        for c in 0..8 {
            let id = r * 8 + c;
            if !seats[id] && seats[id + 1] && seats[id - 1] {
                return id;
            }
        }
    }
    panic!();
}

fn get_id(seat: &str) -> usize {
    let (row, column) = seat.split_at(7);
    get_row(row) * 8 + get_column(column)
}

fn get_row(row: &str) -> usize {
    let mut r = 0;
    for (n, char) in row.chars().rev().enumerate() {
        let bin = match char {
            'F' => 0,
            'B' => 1,
            _ => panic!(),
        };
        r |= bin << n;
    }
    r
}

fn get_column(column: &str) -> usize {
    let mut c = 0;
    for (n, char) in column.chars().rev().enumerate() {
        let bin = match char {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        };
        c |= bin << n;
    }
    c
}

#[test]
fn ex() {
    let a = "BFFFBBFRRR";
    assert_eq!(get_id(a), 567);
}
