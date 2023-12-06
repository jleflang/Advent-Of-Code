use std::time::{Duration, Instant};

use schema::Solver;


pub struct Day06;


impl Solver for Day06 {
    
    fn title(&self) -> &'static str {
        r"Wait For It"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<std::time::Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        unimplemented!()
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<std::time::Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        false
    }
}
