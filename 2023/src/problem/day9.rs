use std::time::{Duration, Instant};
use num::integer::binomial;

use schema::Solver;


pub struct Day09;

impl Solver for Day09 {
    
    fn title(&self) -> &'static str {
        r"Mirage Maintenance"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let lines: Vec<Vec<i64>> = input.lines()
                                        .map(|l| {
                                                    l.split_ascii_whitespace()
                                                    .map(|i| i.parse().unwrap())
                                                    .collect::<Vec<i64>>()
                                                }).collect();

        let mut total = 0i64;
        for line in lines.iter() {

            let mut sign = 1i64;

            let mut next = 0i64;
            for (i, val) in line.iter().enumerate() {
                next += sign * binomial::<i64>(line.len() as i64, i as i64) * val;
                sign *= -1;

            }

            total += next;

        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let mut lines: Vec<Vec<i64>> = input.lines()
                                        .map(|l| {
                                                    l.split_ascii_whitespace()
                                                    .map(|i| i.parse().unwrap())
                                                    .collect::<Vec<i64>>()
                                                }).collect();

        let mut total = 0i64;
        for line in lines.iter_mut() {

            let mut sign = 1i64;

            let mut next = 0i64;
            for (i, val) in line.iter().rev().enumerate() {
                next += sign * binomial::<i64>(line.len() as i64, i as i64) * val;
                sign *= -1;

            }

            total += next;

        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn is_empty(&self) -> bool {
        false
    }
}


#[cfg(test)]
mod test {
    use std::{sync::Arc, sync::Mutex, ops::Deref};

    use schema::*;

    use indoc::indoc;

    use super::Day09;

    const EXP1: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day09.part_a(EXP1, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(114));
    }

}
