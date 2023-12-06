use std::{
    time::{Duration, Instant}, 
    collections::BTreeMap, 
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

    /// INSERT Blade (1998) Quote ABOUT Ice Skating Uphill
    fn funny_walk(&self, loc: u64) -> u64 {

        let mut diff_key = loc;

        for map in self.maps.iter().rev() {

            // v.1 is the range
            // v.0 is the destination mapping
            // key is the source mapping
            for (&k, &v) in map.iter() {
                if (diff_key >= v.0) && (diff_key.saturating_sub(v.0) <= v.1) {
                    diff_key = k + (diff_key - v.0);
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
    
        for s in sects.into_iter().filter(|x| !x.is_empty()) {
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

        let mut loc = almanac.maps.last().unwrap().first_key_value().unwrap().1.0;
        
        for location in almanac.maps.last().unwrap().iter().skip(1) {
            let candidate = almanac.funny_walk(loc);

            for seed_range in almanac.seeds.chunks_exact(2) {
                if candidate > seed_range[0] && candidate <= (seed_range[0] + seed_range[1]) {
                    loc = loc.min(location.1.0);
                }
            }
        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = loc.into();

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
