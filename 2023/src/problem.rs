use schema::Solver;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub const DAYS: &[&dyn Solver] = &[
    &day1::Day01,
    &day2::Day02,
    &day3::Day03,
    &day4::Day04,
    &day5::Day05,
];

