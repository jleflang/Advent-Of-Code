use std::{time::{Duration, Instant}, error::Error};

use schema::Solver;


pub struct Day02;

fn line_parse(line: &str) -> Result<(u32, u32, u32, u32), Box<dyn Error + Send + Sync>> {
    let (game, lgame) = line.split_once(':').unwrap();

    let id = game.split_at(5).1.parse::<u32>().unwrap();

    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for set in lgame.split_terminator(&[';',',']) {
        let mut c = set.split_whitespace();

        let val = c.next().unwrap().parse::<u32>().unwrap();

        match c.next().unwrap_or_default() {
            "red"   => { red = red.max(val);        },
            "green" => { green = green.max(val);    },
            "blue"  => { blue = blue.max(val);      },
            _ => {
                eprintln!("Found invalid color at {id}");
                return Err(Box::<dyn Error + Send + Sync>::from("Bad input!"));
            },
        }

    }

    Ok((id, red, green, blue))

}

impl Solver for Day02 {
    fn title(&self) -> &'static str {
        r"Cube Conundrum"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let mut total = 0;

        let ts = Instant::now();
        for line in input.lines() {
            let (id, red, green, blue) = line_parse(line)?;

            if (red <= 12) && (green <= 13) && (blue <= 14) {
                total += id;
            }

        }
        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let mut total = 0;

        let ts = Instant::now();
        for line in input.lines() {
            let (_, red, green, blue) = line_parse(line)?;

            total += red * green * blue;

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