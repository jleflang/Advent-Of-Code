use std::{
    time::{Duration, Instant}, 
    collections::VecDeque, 
    fmt::Display, cmp::max
};
use ahash::{HashSet, HashSetExt};
use schema::Solver;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N = 0b0001,
    E = 0b0010,
    W = 0b0100,
    S = 0b1000
}

impl Direction {
    fn opposing_end(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::W => Self::E,
            Self::S => Self::N
        }
    }

    fn match_dir(dir: u8) -> Self {
        match dir {
            0b1000 => Self::S,
            0b0010 => Self::E,
            0b0100 => Self::W,
            0b0001 => Self::N,
            _      => unreachable!()
        }
    }

    fn move_dir(&self, pos: Pos) -> Pos {
        match self {
            Self::S => (pos.0 + 1, pos.1),
            Self::N => (pos.0 - 1, pos.1),
            Self::E => (pos.0, pos.1 + 1),
            Self::W => (pos.0, pos.1 - 1)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeTyp {
    Empty,
    MirrorDown,
    MirrorUp,
    SplitTD,
    SplitLR,
}

impl Default for NodeTyp {
    fn default() -> Self {
        Self::Empty
    }
}

impl NodeTyp {
    fn parse(c: u8) -> Self {
        match c {
            b'.'  => Self::Empty,
            b'|'  => Self::SplitTD,
            b'-'  => Self::SplitLR,
            b'\\' => Self::MirrorDown,
            b'/'  => Self::MirrorUp,
            _     => unreachable!()
        }
    }

    fn split(&self) -> Option<(u8, u8)> {
        match *self {
            Self::SplitTD => Some((Direction::N as u8, Direction::S as u8)),
            Self::SplitLR => Some((Direction::E as u8, Direction::W as u8)),
            _             => None,
        }
    }

    fn reflect(&self) -> Option<u8> {
        match *self {
            Self::MirrorDown => Some(Direction::S as u8 | Direction::W as u8),
            Self::MirrorUp   => Some(Direction::N as u8 | Direction::W as u8),
            _                => None,
        }
    }

    fn negative_reflect(&self) -> Option<u8> {
        match *self {
            Self::MirrorDown => Some(Direction::N as u8 | Direction::E as u8),
            Self::MirrorUp   => Some(Direction::S as u8 | Direction::E as u8),
            _                => None,
        }
    }
}

impl Display for NodeTyp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::MirrorDown => write!(f, "\u{005C}"),
            Self::MirrorUp => write!(f, "/"),
            Self::SplitTD => write!(f, "|"),
            Self::SplitLR => write!(f, "-"),
        }
    }
}

#[derive(Debug, Default)]
struct BeamPath {
    beams: VecDeque<(Pos, Direction)>,
}

impl BeamPath {

    /// Add the "head" of the beam
    fn add_node(&mut self, pos: Pos, direction: Direction) {
        self.beams.push_back((pos, direction));
    }

    /// Get a beam position to check
    fn pop_front(&mut self) -> Option<(Pos, Direction)> {
        self.beams.pop_front()
    }

    /// Follow the laser path through the grid until we hit a wall
    fn fire_laser(&mut self, grid: &LaserGrid, beam: &mut Beam) {

        let mut traversed: HashSet<(Pos, u8)> = HashSet::new();

        // Pos => (y, x)
        // (0, 0) => top left, (grid.len()-1, grid[y.max].len()-1) = bottom right
        // N = -1, S = +1
        // W = -1, E = +1
        while let Some((pos, cur_dir)) = self.pop_front() {

            if pos.0 >= grid.len() as i32 || pos.0 < 0 || 
                pos.1 >= grid[pos.0 as usize].len() as i32 || pos.1 < 0 ||
                traversed.contains(&(pos, cur_dir as u8))
            {
                continue;
            }

            traversed.insert((pos, cur_dir as u8));
            beam.insert(pos);
            let found_node = grid[pos.0 as usize][pos.1 as usize];

            match found_node {
                NodeTyp::MirrorDown => {
                                            let mirror_dir = cur_dir.opposing_end();
                                            let next_dir = if cur_dir == Direction::S || cur_dir == Direction::W {
                                                found_node.negative_reflect().unwrap() ^ mirror_dir as u8
                                            }
                                            else {

                                                found_node.reflect().unwrap() ^ mirror_dir as u8
                                            };

                                            let next = Direction::match_dir(next_dir);
                                            self.add_node(next.move_dir(pos), next);
                                        },
                NodeTyp::MirrorUp   => {
                                            let mirror_dir = cur_dir.opposing_end();
                                            let next_dir = if cur_dir == Direction::N || cur_dir == Direction::W {
                                                found_node.negative_reflect().unwrap() ^ mirror_dir as u8
                                            }
                                            else {
                                                found_node.reflect().unwrap() ^ mirror_dir as u8
                                            };

                                            let next = Direction::match_dir(next_dir);
                                            self.add_node(next.move_dir(pos), next);
                                        },
                NodeTyp::SplitLR    => {
                                            if cur_dir != Direction::E || cur_dir != Direction::W {
                                                let (l, r) = found_node.split().unwrap();

                                                let next_l = Direction::match_dir(l);
                                                let next_r = Direction::match_dir(r);

                                                self.add_node(next_l.move_dir(pos), next_l);
                                                self.add_node(next_r.move_dir(pos), next_r);
                                            }
                                            else {
                                                self.add_node(cur_dir.move_dir(pos), cur_dir);
                                            }
                                        },
                NodeTyp::SplitTD    => {
                                            if cur_dir != Direction::N || cur_dir != Direction::S {
                                                let (l, r) = found_node.split().unwrap();

                                                let next_l = Direction::match_dir(l);
                                                let next_r = Direction::match_dir(r);

                                                self.add_node(next_l.move_dir(pos), next_l);
                                                self.add_node(next_r.move_dir(pos), next_r);
                                            }
                                            else {
                                                self.add_node(cur_dir.move_dir(pos), cur_dir);
                                            }
                                        },
                NodeTyp::Empty      => {
                                            self.add_node(cur_dir.move_dir(pos), cur_dir);
                                        }
            }

        }
    }
}

type Pos = (i32, i32);
type Beam = HashSet<Pos>;
type LaserGrid = Vec<Vec<NodeTyp>>;

pub struct Day16;


impl Solver for Day16 {
    fn title(&self) -> &'static str {
        r"The Floor Will Be Lava"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let ts = Instant::now();

        let mut grid = LaserGrid::default();
        let mut beam = Beam::default();

        let mut temp = Vec::<NodeTyp>::new();

        for l in input.lines() {
            temp.clear();
            temp.extend(l.bytes().map(NodeTyp::parse));
            grid.push(temp.clone());
        }

        drop(temp);

        let mut beam_path = BeamPath::default();

        beam_path.add_node((0, 0), Direction::E);
        beam.insert((0, 0));

        // FIRE!!!
        beam_path.fire_laser(&grid, &mut beam);

        // for (y, l) in grid.iter().enumerate() {
        //     for (x, n) in l.iter().enumerate() {
        //         if !beam.contains(&(y as i32,x as i32)) {
        //             eprint!("{n}");
        //         } else {
        //             eprint!("#");
        //         }
        //     }
        //     eprintln!();
        // }
        
        let total = beam.len();
        
        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();

        let mut grid = LaserGrid::default();

        let mut temp = Vec::<NodeTyp>::new();

        for l in input.lines() {
            temp.clear();
            temp.extend(l.bytes().map(NodeTyp::parse));
            grid.push(temp.clone());
        }

        drop(temp);

        let t_max = (0..grid[0].len()).map(|x| {
                        let mut beam = Beam::default();

                        let mut beam_path = BeamPath::default();

                        beam_path.add_node((0, x as i32), Direction::S);
                        beam.insert((0, x as i32));

                        // FIRE!!!
                        beam_path.fire_laser(&grid, &mut beam);

                        beam.len()
                    }).max().unwrap();
        let d_max = (0..grid[0].len()).map(|x| {
                        let mut beam = Beam::default();

                        let mut beam_path = BeamPath::default();

                        beam_path.add_node((grid.len() as i32 - 1, x as i32), Direction::N);
                        beam.insert((grid.len()as i32 - 1, x as i32));

                        // FIRE!!!
                        beam_path.fire_laser(&grid, &mut beam);

                        beam.len()
                    }).max().unwrap();

        let td_max = max(t_max, d_max);

        let l_max = (0..grid.len()).map(|y| {
                        let mut beam = Beam::default();

                        let mut beam_path = BeamPath::default();

                        beam_path.add_node((y as i32, 0), Direction::E);
                        beam.insert((y as i32, 0));

                        // FIRE!!!
                        beam_path.fire_laser(&grid, &mut beam);

                        beam.len()
                    }).max().unwrap();
        let r_max = (0..grid.len()).map(|y| {
                        let mut beam = Beam::default();

                        let mut beam_path = BeamPath::default();

                        beam_path.add_node((y as i32, grid[0].len() as i32 - 1), Direction::W);
                        beam.insert((y as i32, grid[0].len() as i32 - 1));

                        // FIRE!!!
                        beam_path.fire_laser(&grid, &mut beam);

                        beam.len()
                    }).max().unwrap();

        let lr_max = max(l_max, r_max);

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = max(td_max, lr_max).into();

        Ok(d)
    }

    fn is_empty(&self) -> bool {
        false
    }
}


#[cfg(test)]
mod test {
    use std::{sync::Arc, sync::Mutex};

    use schema::*;

    use indoc::indoc;

    use super::Day16;

    const GRID: &str = indoc! {"
        .|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|....
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day16.part_a(GRID, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(46));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day16.part_b(GRID, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(10));
    }

}
