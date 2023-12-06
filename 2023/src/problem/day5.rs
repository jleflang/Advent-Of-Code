use std::{
    time::{Duration, Instant}, 
    collections::{BTreeMap, VecDeque}, 
};

use schema::Solver;

/// The core entity for all the mappings
/// Data is stored as key = source, value = (destination, range)
type Map = BTreeMap<u64, (u64, u64)>;

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,

    maps: Vec<Map>
}

impl Almanac {
    
    /// Walks the tree to find the next key.
    fn walk(&self, seed: u64) -> u64 {

        let mut diff_key = seed;

        for map in self.maps.iter() {

            // v.1 is the range
            // v.0 is the destination mapping
            // key is the source mapping
            for (&k, &v) in map.iter() {
                if (diff_key >= k) && (diff_key.saturating_sub(k) <= v.1) {
                    diff_key = v.0 + (diff_key - k);
                    break;
                }

            }

        }

        diff_key
    }

    fn build_almanac(input: &str) -> Self {
    
        let mut sects = input.split("\n\n");
    
        let seeds = sects.next()
                                .unwrap()
                                .split_whitespace()
                                .skip(1)
                                .map(|m| m.parse().unwrap())
                                .collect();
    
        let mut maps = Vec::<Map>::new();
    
        for s in sects.filter(|x| !x.is_empty()) {
            let lines = s.lines();
    
            let mut map = Map::new();
    
            for l in lines.skip(1) {
                let mut parts = l.split_whitespace();
    
                let dest: u64 = parts.next().unwrap().parse().unwrap();
                let source: u64 = parts.next().unwrap().parse().unwrap();
                let range: u64 = parts.next().unwrap().parse().unwrap();
    
                map.insert(source, (dest, range));
            }
    
            maps.push(map);
    
        }
    
        Almanac { seeds, maps }
    
    }
}



pub struct Day05;

impl Solver for Day05 {
    fn title(&self) -> &'static str {
        r"If You Give A Seed A Fertilizer"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let almanac = Almanac::build_almanac(input);

        let mut location = u64::MAX;

        for seed in almanac.seeds.iter() {
            location = location.min(almanac.walk(*seed));
        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = location.into();

        Ok(d)
        
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let almanac = Almanac::build_almanac(input);

        // Co-opted this solution:
        // https://github.com/Fabi019/aoc2023/blob/main/src/bin/day05.rs#L38
        let mut seeds: Vec<(u64, u64)> = VecDeque::new().into();
        seeds.extend(almanac.seeds.chunks_exact(2).map(|s| (s[0], s[0] + s[1])));

        for map in almanac.maps.iter() {

            let map_ranges: Vec<(u64, u64, u64)> = map.iter().map(|(s, (d, r))| (*d, *s, *s+*r)).collect();

            let mut queue: VecDeque<(u64, u64)> = VecDeque::new();
            queue.extend(seeds.iter());

            let mut new_q: Vec<(u64, u64)> = Vec::new();

            while let Some((start, end)) = queue.pop_front() {
                let mut found = false;

                for &(target, lower, upper) in &map_ranges {
                    if start >= lower && start < upper && end < upper {
                        // Complete range is in bounds
                        let s = start + target - lower;
                        let e = end + target - lower;
                        new_q.push((s, e));
                        found = true;
                        break;
                    } else if start >= lower && start < upper {
                        // Start is in bounds, end is not
                        let s = start + target - lower;
                        let e = upper - 1 + target - lower;
                        new_q.push((s, e));
                        queue.push_back((upper, end));
                        found = true;
                        break;
                    } else if start < lower && end >= lower && end < upper {
                        // End is in bounds, start is not
                        let s = lower + target - lower;
                        let e = end + target - lower;
                        new_q.push((s, e));
                        queue.push_back((start, lower - 1));
                        found = true;
                        break;
                    } else if start < lower && end >= upper {
                        // Neither start nor end are in bounds
                        new_q.push((lower + target - lower, upper - 1 + target - lower));
                        queue.push_back((upper, end));
                        queue.push_back((start, lower - 1));
                        found = true;
                        break;
                    }
                }

                if !found {
                    // No overlap with any range
                    new_q.push((start, end));
                }
            }

            seeds = new_q;
        
        }

        let location = seeds.iter().min().unwrap().0;

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = location.into();

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

    use super::Day05;

    const PROBLEM: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day05.part_a(PROBLEM, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(35));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day05.part_b(PROBLEM, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(46));
    }
}
