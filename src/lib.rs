#![feature(min_const_generics)]
#![feature(const_in_array_repeat_expressions)]
#![feature(deque_range)]
#![feature(destructuring_assignment)]
#![feature(str_split_once)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_lib! { year = 2020 }
