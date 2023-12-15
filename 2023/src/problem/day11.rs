use std::time::{Duration, Instant};
use vob::{vob, Vob};
use itertools::Itertools;

use schema::Solver;

type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Galaxy {
    position: Pos,
}

impl Galaxy {
    #[inline]
    fn pos(&self) -> (usize, usize) {
        self.position
    }
}

#[inline(always)]
fn distance(a: [usize;2], b: [usize;2]) -> usize {
    (a[0].abs_diff(b[0])) + (a[1].abs_diff(b[1]))
}

type Map = Vec<Galaxy>;

#[derive(Debug)]
struct Universe {
    x: Vob,
    y: Vob
}

impl Universe {

    fn hubble_expansion(&self, times: usize, map: &mut Map) {
        let expand = times - 1;

        for (my, _) in self.y.iter().enumerate().rev().filter(|coord| !coord.1) {
            for p in map.iter_mut().filter(|g| g.pos().1 > my) {
                p.position.1 += expand;
            }
        }

        for (mx, _) in self.x.iter().enumerate().rev().filter(|coord| !coord.1) {
            for p in map.iter_mut().filter(|g| g.pos().0 > mx) {
                p.position.0 += expand;
            }
        }
    }

}


pub struct Day11;

impl Solver for Day11 {
    
    fn title(&self) -> &'static str {
        r"Cosmic Expansion"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let lines = input.lines().collect::<Vec<_>>();

        let mut mx = vob![false; lines[0].bytes().into_iter().count()];
        let mut my = vob![false; lines.len()];

        let mut map: Map = Vec::new();

        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.bytes().enumerate() {
                if c == b'#' {
                    map.push(Galaxy { position: (x, y) });

                    mx.set(x, true);
                    my.set(y, true);
                }
            }
        }

        let universe = Universe { x: mx, y: my };

        universe.hubble_expansion(2, &mut map);

        let dist: usize = map.iter()
                             .map(|g| [g.pos().0, g.pos().1])
                             .tuple_combinations()
                             .map(|(g1, g2)| distance(g1, g2))
                             .sum();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = dist.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();
        let lines = input.lines().collect::<Vec<_>>();

        let mut mx = vob![false; lines[0].bytes().into_iter().count()];
        let mut my = vob![false; lines.len()];

        let mut map: Map = Vec::new();

        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.bytes().enumerate() {
                if c == b'#' {
                    map.push(Galaxy { position: (x, y) });

                    mx.set(x, true);
                    my.set(y, true);
                }
            }
        }

        let universe = Universe { x: mx, y: my };

        universe.hubble_expansion(1_000_000, &mut map);

        let dist: usize = map.iter()
                             .map(|g| [g.pos().0, g.pos().1])
                             .tuple_combinations()
                             .map(|(g1, g2)| distance(g1, g2))
                             .sum();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = dist.into();

        Ok(d)
    }

    fn is_empty(&self) -> bool {
        false
    }
}
