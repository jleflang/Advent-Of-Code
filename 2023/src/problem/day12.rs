use std::time::{Duration, Instant};


use schema::Solver;


pub struct Day12;

/// Heavily inspired by <https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day12.rs>
fn solve(springs: &Vec<u8>, rep: usize, count: &Vec<usize>) -> usize {
    
    let mut spring_pat = Vec::with_capacity(springs.len() * rep + 1);
    let mut spring_count = Vec::with_capacity(count.len() * rep);

    for _ in 1..rep {
        spring_pat.extend_from_slice(&springs);
        spring_pat.push(b'?');
        spring_count.extend_from_slice(&count);
    };

    spring_pat.extend_from_slice(&springs);
    spring_count.extend_from_slice(&count);

    spring_pat.push(b'.');

    let mut broke = vec![0; 200];
    let mut tabula = vec![0; 10_000];

    let mut sum = 0;

    for (i, &s) in spring_pat.iter().enumerate() {
        if s != b'.' {
            sum += 1;
        }

        broke[i + 1] = sum;
    }

    let wiggle = spring_pat.len() - spring_count.iter().fold(0, |a, i| a + *i) - spring_count.len() + 1;

    let size = spring_count[0];
    let mut sum = 0;
    let mut v = true;

    for i in 0..wiggle {
        if spring_pat[i + size] == b'#' {
            sum = 0;
        }
        else if v && broke[i + size] - broke[i] == size {
            sum += 1;
        }

        tabula[i + size] = sum;

        v &= spring_pat[i] != b'#';
    }

    let mut start = size + 1;

    for (r, &s) in spring_count.iter().enumerate().skip(1) {
        let prev = (r - 1) * spring_pat.len();
        let cur = r * spring_pat.len();

        sum = 0;

        for p in start..start + wiggle {

            if spring_pat[p + s] == b'#' {
                sum = 0;
            }
            else if tabula[prev + p - 1] > 0 
                && spring_pat[p - 1] != b'#' 
                && broke[p + s] - broke[p] == s {
                sum += tabula[prev + p - 1];
            }

            tabula[cur + p + s] = sum;

        }

        start += s + 1;

    }

    sum
}


impl Solver for Day12 {
    fn title(&self) -> &'static str {
        r"Hot Springs"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let mut total = 0;

        let mut pattern: Vec<usize> = Vec::new();
        let mut spring_pat: Vec<u8> = Vec::new();

        for line in input.lines() {
            if let Some((springs, arrangement)) = line.split_once(' ') {

                pattern.clear();
                pattern = arrangement.split(',').map(|v| v.parse().unwrap()).collect();

                spring_pat.clear();
                spring_pat = springs.bytes().collect();

                let temp = solve(&spring_pat, 1, &pattern);

                total += temp;

            }
        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let mut total = 0;

        let mut pattern: Vec<usize> = Vec::new();
        let mut spring_pat: Vec<u8> = Vec::new();

        for line in input.lines() {
            if let Some((springs, arrangement)) = line.split_once(' ') {

                pattern.clear();
                pattern = arrangement.split(',').map(|v| v.parse().unwrap()).collect();

                spring_pat.clear();
                spring_pat = springs.bytes().collect();

                let temp = solve(&spring_pat, 5, &pattern);

                total += temp;

            }
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

    use super::Day12;

    const SPRINGS: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day12.part_a(SPRINGS, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(21));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day12.part_b(SPRINGS, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(525152));
    }

}
