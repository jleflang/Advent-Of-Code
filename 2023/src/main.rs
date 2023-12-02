use std::{error::Error, sync::{Arc, Mutex}, };

use clap::{Parser, Subcommand};

mod problem;


/// Advent of Code top level
#[derive(Parser)]
#[command(name = "Advent of Code", author = "James Leflang")]
struct Cli {
    /// Set the year
    #[command(subcommand)]
    day: Days,
}

#[derive(Subcommand, Clone)]
enum Days {
    /// Valid Years
    Day {
        day: u32,

        part: Option<String>
    },

    /// View Years
    List,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();

    match args.day {
        Days::Day { day, part } => {

            let input = {
                let f = format!("./input/day{}", day);
                
                std::fs::read_to_string(f)?
            };

            let day = day.saturating_sub(1);

            let solution = match problem::DAYS.get(day as usize) {
                Some(p) => p,
                None => {
                    eprintln!("[ERROR] {day} not found!");
                    return Err("Invalid day selected!".into());
                },
            };

            if solution.is_empty() {
                return Err("Empty Solution!".into());
            }

            let out_a = Arc::new(Mutex::new(schema::Answer::Unimplemented));
            let out_b = Arc::new(Mutex::new(schema::Answer::Unimplemented));

            println!("--------------------------------------");
            println!("Day {}: {}", day + 1, solution.title());
            println!("--------------------------------------");

            match part.unwrap().to_lowercase().as_str() {
                "a"     => {
                                let out_a_clone = out_a.clone();

                                let t = std::thread::spawn(move || 
                                    solution.part_a(&input, out_a_clone)
                                )
                                .join()
                                .expect("Part A Errored");
                                assert!(t.is_ok());

                                println!("Part A answer: {} (took {:#?})", out_a.lock().unwrap(), t.unwrap());
                            },
                "b"     => {
                                let out_b_clone = out_b.clone();

                                let t = std::thread::spawn(move || 
                                    solution.part_b(&input, out_b_clone)
                                )
                                .join()
                                .expect("Part B Errored");
                                assert!(t.is_ok());

                                println!("Part B answer: {} (took {:#?})", out_b.lock().unwrap(), t.unwrap());
                            },
                "all"   => std::thread::scope(|s| {
                                let out_a_clone = out_a.clone();
                                let out_b_clone = out_b.clone();

                                let a = s.spawn(|| solution.part_a(&input, out_a_clone));
                                let b = s.spawn(|| solution.part_b(&input, out_b_clone));

                                let d_a = a.join().expect("Part A Thread Died").unwrap();
                                let d_b = b.join().expect("Part B Thread Died").unwrap();

                                println!("Part A answer: {} (took {:#?})", out_a.lock().unwrap(), d_a);
                                println!("Part B answer: {} (took {:#?})", out_b.lock().unwrap(), d_b);
                            }),
                _       => return Err("Invalid Part Selection!".into()),
            }

        },
        Days::List => unimplemented!(),
    }

    Ok(())
}
