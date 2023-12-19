use std::time::{Duration, Instant};
use vob::Vob;

use schema::Solver;

type Mirror = Vec<Vob>;

fn parse_mirror(input: &str) -> Mirror {

    let mut p = Mirror::new();

    for l in input.lines() {
        let mut li: Vob = Vob::with_capacity(l.len());

        for c in l.as_bytes().iter() {
            if *c == b'#' {
                li.push(true);
            } else {
                li.push(false);
            }
        }
        p.push(li);
    }

    p

}

fn transpose(mirror: &Mirror) -> Mirror {

    let mut new_mirror = Mirror::with_capacity(mirror[0].len());
    let mut col: Vob = Vob::with_capacity(mirror.len());

    for i in 0..mirror[0].len() {
        col.clear();
        col.extend(mirror.iter().map(|v| v[i]));
        new_mirror.push(col.clone());
    }

    new_mirror

}

#[derive(Debug, Default)]
struct Reflections {
    vertical: usize,
    horizontal: usize,
}

impl Reflections {

    fn vertical_reflect(&mut self, mirror: &Mirror, allowed: usize) -> &mut Self {
    
        let t_mirror = transpose(mirror);

        self.vertical = Self::reflect(&t_mirror, allowed);

        self
    }

    fn horizontal_reflect(&mut self, mirror: &Mirror, allowed: usize) -> &mut Self {

        self.horizontal = Self::reflect(mirror, allowed);

        self
    }

    fn reflect(mirror: &Mirror, allowed: usize) -> usize {
        let mut total = 0;

        for i in 1..mirror.len() {
            let (left, right) = mirror.split_at(i);

            let left: Vec<Vob> = left.iter().rev().map(|v| v.clone()).collect();

            if left.len() > right.len() {
                let short_len = right.len();

                let s = left[..short_len].iter()
                                         .zip(right.iter())
                                         .map(|(v1, v2)| 
                                                  (v1 ^ v2).iter_set_bits(..).count()
                                             )
                                         .sum::<usize>();

                if s == allowed {
                    total = i;
                    break;
                }
            } else {
                let short_len = left.len();

                let s = right[..short_len].iter()
                                          .zip(left.iter())
                                          .map(|(v1, v2)| 
                                                  (v1 ^ v2).iter_set_bits(..).count()
                                              )
                                          .sum::<usize>();

                if s == allowed {
                    total = i;
                    break;
                }
            };

        }

        total

    }

    #[inline(always)]
    fn calculate(&self) -> usize {
        self.vertical + self.horizontal * 100
    }

}

pub struct Day13;


impl Solver for Day13 {
    fn title(&self) -> &'static str {
        r"Point of Incidence"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let mirrors = input.split("\n\n");
        let total: usize =  mirrors.map( |mirror| {
                                            let p = parse_mirror(mirror);
                                            Reflections::default().horizontal_reflect(&p, 0)
                                                                  .vertical_reflect(&p, 0)
                                                                  .calculate()
                                        }).sum();

        let d = ts.elapsed();
        
        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let mirrors = input.split("\n\n");
        let total: usize =  mirrors.map( |mirror| {
                                            let p = parse_mirror(mirror);
                                            Reflections::default().horizontal_reflect(&p, 1)
                                                                  .vertical_reflect(&p, 1)
                                                                  .calculate()
                                        }).sum();

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

    use super::Day13;

    const MIRROR: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day13.part_a(MIRROR, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(405));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day13.part_b(MIRROR, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(400));
    }
}
