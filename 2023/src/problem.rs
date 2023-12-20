use schema::Solver;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

pub const DAYS: &[&dyn Solver] = &[
    &day1::Day01,
    &day2::Day02,
    &day3::Day03,
    &day4::Day04,
    &day5::Day05,
    &day6::Day06,
    &day7::Day07,
    &day8::Day08,
    &day9::Day09,
    &day10::Day10,
    &day11::Day11,
    &day12::Day12,
    &day13::Day13,
    &day14::Day14,
    &day15::Day15,
];

