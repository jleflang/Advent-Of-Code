use std::time::{Duration, Instant};
use ahash::AHashMap;

use schema::Solver;
use crate::problem::day3::Found::*;

#[derive(Debug, Default, Clone, Copy, Hash)]
struct Position {
    start: (u32, u32),
    end: (u32, u32),
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start) && (self.end == self.end)
    }

    fn ne(&self, other: &Self) -> bool {
        (self.start != other.start) && (self.end != self.end)
    }
}

impl Eq for Position {}

enum Found {
    Part(u32),
    Ratio(u32),
    Uninit,
}

impl Found {
    
    fn new() -> Self {
        Found::Uninit
    }

    fn get_part(&self) -> u32 {
        match *self {
            Part(ref u32) => *u32,
            _ => 0,
        }
    }

    fn get_ratio(&self) -> u32 {
        match *self {
            Ratio(ref u32) => *u32,
            _ => 0,
        }
    }

}

impl Default for Found {
    fn default() -> Self {
        Found::new()
    }
}

/// An interesting algorithm to find all the part numbers and the parts in the
/// input, then for each part look around and find the part numbers.
/// 
/// In the case of gears, the filter is easier!
fn find_parts(page: &str, is_ratio: bool) -> Found {

    let mut parts: AHashMap<(u32, u32), char> = AHashMap::new();
    let mut numbers: AHashMap<Position, u32> = AHashMap::new();
    let mut pos = Position::default();
    let mut is_num = false;
    let mut it = 0;
    let mut max_x = 0;

    let p = page.lines();
    let max_y = p.clone().count();
    
    for (y, l) in p.enumerate() {

        max_x = max_x.max(l.char_indices().count());

        for (x, s) in l.char_indices() {

            // An awkward way of handling the literal edge cases
            if is_num && (x == 0) {
                pos.end = ((max_x-1) as u32, ((y-1) as u32));
                numbers.insert(pos, it);

                is_num = false;
                it = 0;
                pos = Position::default();
            }

            if s.is_ascii_punctuation() && (s != '.') && !is_num {
                parts.insert(((x as u32), (y as u32)), s);
            }
            else if s.is_ascii_digit() {
                if !is_num {
                    is_num |= true;
                    pos.start = ((x as u32), (y as u32));
                }
                it = (it << 3) + (it << 1);
                it += s.to_digit(10).unwrap();
            }
            else if is_num && !s.is_ascii_digit() {
                if s != '.' {
                    parts.insert(((x as u32), (y as u32)), s);
                }

                pos.end = ((x as u32).saturating_sub(1), (y as u32));
                numbers.insert(pos, it);

                is_num = false;
                it = 0;
                pos = Position::default();
            }
        }
    }

    if !is_ratio {
        // This resembles bounding box aabb detection
        let found_value = |r: Position, i| {
            let (x_s, y_s) = r.start;
            let (w, h) = r.end;
            let mut a = None;
            for x in x_s.saturating_sub(1)..=(w+1).min(max_x as u32) {
                for y in y_s.saturating_sub(1)..=(h+1).min(max_y as u32) {
                    if parts.contains_key(&(x, y)) {
                        a = Some(i);
                        break;
                    }
                }
            };
    
            a
        };

        let ans: u32 = numbers.iter()
                        .filter_map(|(t, i)| found_value(*t, *i))
                        .sum::<u32>();
        Found::Part(ans)
    }
    else {
        let ans = 0;
        Found::Ratio(ans)
    }
}

pub struct Day03;

impl Solver for Day03 {

    fn title(&self) -> &'static str {
        r"Gear Ratios"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let ts = Instant::now();
        let ans = find_parts(input, false).get_part();
        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = ans.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let ts = Instant::now();
        let ans = find_parts(input, true).get_ratio();
        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = ans.into();

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

    use super::Day03;

    const PUZZLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    const PUZZLE_ONE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*11.111
        2....+.58.
        ..592.....
        ....8.755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day03.part_a(PUZZLE, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(4361));
    }

    #[test]
    fn part_a_mod() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day03.part_a(PUZZLE_ONE, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(4491));
    }
}