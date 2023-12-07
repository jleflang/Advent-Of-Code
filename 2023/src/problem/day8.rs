use std::{
    time::{Duration, Instant}, 
};

use schema::Solver;


pub struct Day08;

impl Solver for Day08 {
    fn title(&self) -> &'static str {
        r""
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        unimplemented!()
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        true
    }
}