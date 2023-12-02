use std::time::{Duration, Instant};

use schema::Solver;


pub struct Day01;

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", 
    "five", "six", "seven", "eight", "nine"
];

#[inline(always)]
fn get_num(l: &str) -> u32 {
    let num: Vec<u32> = l.chars().filter_map(|s| s.to_digit(10)).collect();

    let mut first = *num.first().unwrap();
    first = (first << 3) + (first << 1);
    first + num.last().unwrap_or(&first)
}

/// A naive replacement algorithm that replaces the first letter of digit
/// in a line with the numerical value.
fn naive_replace(line: &mut str, digit: &str, val: &str) {

    let chars = unsafe {line.as_bytes_mut() };

    let mut i = 0;

    while i < chars.len() {
        if chars[i..].starts_with(digit.as_bytes()) {
            chars[i+1] = val.as_bytes()[0];
        }

        i += 1;
    }

}

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
            total += get_num(l)
        }
        let ts = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(ts)
    }

    /// Part 2: ...
    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<&(dyn std::error::Error + Send + Sync)>> {

        let mut total = 0;

        let ts = Instant::now();

        for l in input.lines() {
            let mut mod_line: String = l.to_string();

            for (i, digit) in DIGITS.iter().enumerate() {
                naive_replace(mod_line.as_mut_str(), digit, format!("{}", i).as_str());
            }

            eprintln!("{mod_line}");

            total += get_num(mod_line.as_str())
        }
        let ts = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(ts)
    }

    fn is_empty(&self) -> bool {
        false
    }

}

#[cfg(test)]
mod test {

    use std::{sync::{Arc, Mutex}, ops::Deref};

    use schema::Solver;
    use indoc::indoc;

    use super::Day01;

    const CASE_A: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const CASE_B: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_a () {
        let out = Arc::new(Mutex::new(schema::Answer::Unimplemented));
        let _ = Day01.part_a(CASE_A, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), schema::Answer::Number(142));
    }

    #[test]
    fn part_b () {
        let out = Arc::new(Mutex::new(schema::Answer::Unimplemented));
        let _ = Day01.part_b(CASE_B, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), schema::Answer::Number(281));
    }


}
