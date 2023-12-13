use std::time::{Duration, Instant};
use ahash::{HashMap, HashMapExt};
use num::integer::lcm;

use schema::Solver;

fn parse(input: &str) -> (Map, Vec<char>) {
    let mut lines = input.split("\n").filter(|l| !l.is_empty());

    let path: Vec<char> = lines.next().unwrap().chars().collect();

    let mut map: Map = HashMap::new();

    for l in lines {
        let (node, fork) = l.split_once(" = ").unwrap();
        let (left, right) = fork.strip_prefix("(").unwrap()
                                .strip_suffix(")").unwrap()
                                .split_once(", ").unwrap();

        map.insert(node, (left, right));
    }

    (map, path)
}


type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub struct Day08;

impl Solver for Day08 {
    fn title(&self) -> &'static str {
        r"Haunted Wasteland"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let (map, path) = parse(input);

        let mut visited: Vec<&str> = Vec::new();
        let mut node = map.get_key_value("AAA").unwrap();

        for direction in path.iter().cycle() {

            if *node.0 == "ZZZ" {
                break;
            }

            visited.push(*node.0);
            
            let (l, r) = *node.1;

            if direction == &'L' {
                node = map.get_key_value(l).unwrap();
            }
            else {
                node = map.get_key_value(r).unwrap();
            }

        }

        let total = visited.len();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();

        let (map, path) = parse(input);

        let mut path_len = Vec::new();

        let a_nodes = map.iter().filter(|(&k, &(_, _))| k.ends_with('A'));

        for start_node in a_nodes {

            let mut visited: Vec<&str> = Vec::new();

            let mut node = start_node;

            for direction in path.iter().cycle() {

                if node.0.ends_with('Z') {
                    break;
                }

                visited.push(*node.0);
                
                let (l, r) = *node.1;

                if *direction == 'L' {
                    node = map.get_key_value(l).unwrap();
                }
                else {
                    node = map.get_key_value(r).unwrap();
                }

            }

            path_len.push(visited.len() as u64);
        }

        let mut _total = u64::MAX;

        if path_len.len() == 1 {
            _total = path_len[0] as u64;
        }
        else {
            let node_path = path.len() as u64;
            let mut multiple = lcm(path_len.pop().unwrap(), path_len.pop().unwrap()) / node_path;

            while path_len.len() >= 1 {
                multiple = lcm(multiple, path_len.pop().unwrap() / node_path);
            }

            _total = multiple * node_path;

        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = _total.into();

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

    use super::Day08;

    const PATH1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)

    "};

    const PATH2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    
    "};

    const PATHB: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    
    "};

    #[test]
    fn part_a1() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day08.part_a(PATH1, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(2));
    }

    #[test]
    fn part_a2() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day08.part_a(PATH2, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(6));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day08.part_b(PATHB, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(6));
    }
}
