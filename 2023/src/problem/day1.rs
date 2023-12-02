use std::time::{Duration, Instant};

use schema::Solver;


pub struct Day01;

impl Solver for Day01 {

    fn title(&self) -> &'static str {
        r"Trebuchet?!"
    }
    
    /// Part 1: Finds the first and last numerical value in a string,
    /// converts it to a valid two digit number, and sums all values for the
    /// `input` file and stores in `out`.
    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<&(dyn std::error::Error + Send + Sync)>> {
        
        let mut total = 0;

        let ts = Instant::now();
        for l in input.lines() {
            let num: Vec<u32> = l.chars().filter_map(|s| s.to_digit(10)).collect();

            let mut first = *num.first().unwrap();
            first = (first << 3) + (first << 1);
            total += first + num.last().unwrap_or(&first)
        }
        let ts = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(ts)
    }

    /// Part 2: ...
    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<&(dyn std::error::Error + Send + Sync)>> {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        false
    }

}
