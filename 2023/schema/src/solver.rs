use std::{sync::{ Arc, Mutex, }, time::Duration};

use crate::Answer;

pub trait Solver: Send + Sync {
    fn title(&self) -> &'static str;

    fn part_a(&self, input: &str, out: Arc<Mutex<Answer>>) 
        -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>>;

    fn part_b(&self, input: &str, out: Arc<Mutex<Answer>>) 
        -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>>;
    
    fn is_empty(&self) -> bool;
}

pub struct EmptySolver;

impl Solver for EmptySolver {
    fn title(&self) -> &'static str {
        r""
    }

    fn part_a(&self, _input: &str, _out: Arc<Mutex<Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        Ok(Duration::new(0, 0))
    }

    fn part_b(&self, _input: &str, _out: Arc<Mutex<Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        Ok(Duration::new(0, 0))
    }

    fn is_empty(&self) -> bool {
        true
    }
}
