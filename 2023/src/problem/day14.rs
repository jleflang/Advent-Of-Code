use std::time::{Duration, Instant};

use schema::Solver;


type Dish = Vec<Vec<u8>>;
type Patterns = Vec<Dish>;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct TiltBox {
    dish: Dish,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    West,
    East,
    South
}

impl TiltBox {
    fn tilt(&mut self, direction: Direction) -> &mut Self {

        let mut rocks = self.rocks();

        match direction {
            Direction::North => {
                                    rocks.reverse();
                                    while let Some((y, x)) = rocks.pop() {
                                        let mut y_i = y;
                                        let mut rock = 0;

                                        while y_i > 0 {
                                            match self.dish[y_i - 1][x] {
                                                b'O' => rock += 1,
                                                b'#' => break,
                                                _ => ()
                                            }

                                            y_i -= 1;
                                        }

                                        self.dish[y][x] = b'.';
                                        self.dish[y_i+rock][x] = b'O';
                                    }
                                },
            Direction::West => {
                                    rocks.reverse();
                                    while let Some((y, x)) = rocks.pop() {
                                        let mut x_i = x;
                                        let mut rock = 0;

                                        while x_i > 0 {
                                            match self.dish[y][x_i - 1] {
                                                b'O' => rock += 1,
                                                b'#' => break,
                                                _ => ()
                                            }

                                            x_i -= 1;
                                        }

                                        self.dish[y][x] = b'.';
                                        self.dish[y][x_i+rock] = b'O';
                                    }
                                },
            Direction::East => {
                                    while let Some((y, x)) = rocks.pop() {
                                        let mut x_i = x;
                                        let mut rock = 0;

                                        while x_i < self.dish[y].len()-1 {
                                            match self.dish[y][x_i + 1] {
                                                b'O' => rock += 1,
                                                b'#' => break,
                                                _ => ()
                                            }

                                            x_i += 1;
                                        }

                                        self.dish[y][x] = b'.';
                                        self.dish[y][x_i-rock] = b'O';
                                    }
                                },
            Direction::South => {
                                    while let Some((y, x)) = rocks.pop() {
                                        let mut y_i = y;
                                        let mut rock = 0;

                                        while y_i < self.dish.len()-1 {
                                            match self.dish[y_i + 1][x] {
                                                b'O' => rock += 1,
                                                b'#' => break,
                                                _ => ()
                                            }

                                            y_i += 1;
                                        }

                                        self.dish[y][x] = b'.';
                                        self.dish[y_i-rock][x] = b'O';
                                    }
                                },
        }

        self
    }

    /// We go SPIN!!!
    fn cycle(&mut self) {
        self.tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East);
    }

    fn rocks(&self) -> Vec<(usize, usize)> {
        let w = self.dish[0].len();
        self.dish.iter()
                 .flatten()
                 .enumerate()
                 .filter_map(|(v, &i)| 
                    if i == b'O' { 
                        Some((v / w, v % w))
                    } else { 
                        None 
                    })
                 .collect::<Vec<(usize, usize)>>()
    }

    fn load(&self) -> usize {
        self.rocks().iter().map(|(y, _)| self.dish.len() - y).sum()
    }
}

pub struct Day14;


impl Solver for Day14 {
    fn title(&self) -> &'static str {
        r"Parabolic Reflector Dish"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let mut tiltbox = TiltBox::default();

        for l in input.trim().lines() {
            let li = l.bytes().collect::<Vec<u8>>();
            tiltbox.dish.push(li);
        }

        tiltbox.tilt(Direction::North);

        let load = tiltbox.load();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = load.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let mut tiltbox = TiltBox::default();
        let mut pattern = Patterns::new();

        for l in input.trim().lines() {
            let li = l.bytes().collect::<Vec<u8>>();
            tiltbox.dish.push(li);
        }

        pattern.push(tiltbox.dish.clone());

        for _ in 0..1_000_000_000 {

            tiltbox.cycle();

            if pattern.contains(&tiltbox.dish) {
                let idx = pattern.iter().position(|d| *d == tiltbox.dish).unwrap();
                let cycles = pattern.len() - idx;
                let closed_cycle = idx + (1_000_000_000 - idx) % cycles;
                tiltbox.dish = pattern[closed_cycle].clone();
                break;
            }

            pattern.push(tiltbox.dish.clone());
        }

        let load = tiltbox.load();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = load.into();

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

    use super::Day14;

    const DISH: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day14.part_a(DISH, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(136));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day14.part_b(DISH, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(64));
    }

}
