use std::time::{Duration, Instant};

use schema::Solver;


pub struct Day06;


impl Solver for Day06 {
    
    fn title(&self) -> &'static str {
        r"Wait For It"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let mut lines = input.lines();

        let times: Vec<u32> = lines.next().unwrap()
                                        .split_ascii_whitespace()
                                        .skip(1)
                                        .map(|s| s.parse().unwrap())
                                        .collect();

        let dist: Vec<u32> = lines.next().unwrap()
                                        .split_ascii_whitespace()
                                        .skip(1)
                                        .map(|s| s.parse().unwrap())
                                        .collect();

        let ways: u32 = times.iter().zip(dist.iter())
                            .map(|(t, d)| {
                                    let i: u32 = f32::ceil((*t as f32 - f32::sqrt(t.pow(2) as f32 - 4. * *d as f32)) / 2.) as u32;
                                    t + 1 - 2 * i
                                })
                            .product();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = ways.into();


        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let ts = Instant::now();
        let mut lines = input.lines();

        let time: u64 = lines.next().unwrap()
                                        .split_ascii_whitespace()
                                        .skip(1)
                                        .flat_map(|s| s.chars())
                                        .collect::<String>()
                                        .parse::<u64>().unwrap();

        let dist: u64 = lines.next().unwrap()
                                        .split_ascii_whitespace()
                                        .skip(1)
                                        .flat_map(|s| s.chars())
                                        .collect::<String>()
                                        .parse::<u64>().unwrap();

        let i1: u64 = f64::ceil((time as f64 - f64::sqrt(time.pow(2) as f64 - 4. * dist as f64)) / 2.) as u64;
        let i2: u64 = f64::floor((time as f64 + f64::sqrt(time.pow(2) as f64 - 4. * dist as f64)) / 2.) as u64;
        let ways = i2 - i1 + 1;

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = ways.into();

        Ok(d)
    }

    fn is_empty(&self) -> bool {
        false
    }
}
