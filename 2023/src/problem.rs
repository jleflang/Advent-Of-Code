use schema::Solver;

mod day1;
mod day2;

pub const DAYS: &[&dyn Solver] = &[
    &day1::Day01,
    &day2::Day02,
];

