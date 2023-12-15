use std::{
    time::{Duration, Instant}, 
    collections::{BTreeMap, VecDeque}
};

use schema::Solver;

const START_DIR: [((i32, i32), Direction); 4] = 
    [((0, -1), Direction::N), ((-1, 0), Direction::W), ((0, 1), Direction::S), ((1, 0), Direction::E)];

#[derive(Debug, Clone)]
struct Stream {
    depth: u32,
    nodes: VecDeque<(Pos, u8)>
}

#[derive(Debug)]
struct PipeMap {
    map: [[Pipe; 140]; 140],
    start: Pos,
}

impl PipeMap {
    fn new() -> Self {
        Self { map: [[Pipe::Ground; 140]; 140], start: (0, 0) }
    }

    fn tile(&mut self, xy: Pos, typ: char) {

        let t = Pipe::parse(typ);

        if t == Pipe::Start {
            self.start = xy;
        }

        self.map[xy.0 as usize][xy.1 as usize] = t;
    }

    fn next(&self, cur: (Pos, u8)) -> (Pos, u8) {

        let cur_pos = cur.0;
        let facing = Direction::match_dir(cur.1);

        let next_pos = match facing {
            Direction::N => {(cur_pos.0, cur_pos.1.saturating_sub(1))},
            Direction::E => {(cur_pos.0+1, cur_pos.1)},
            Direction::W => {(cur_pos.0.saturating_sub(1), cur_pos.1)},
            Direction::S => {(cur_pos.0, cur_pos.1+1)},
        };

        let pipe = self.map[next_pos.0 as usize][next_pos.1 as usize];

        if pipe == Pipe::Start {
            return (self.start, facing as u8);
        }

        let direction = pipe.ends() ^ facing.opposing_end() as u8;

        (next_pos, direction)

    }

    fn start_paths(&self) -> Stream {
        let mut dir = VecDeque::new();

        for (uv, direction) in START_DIR.iter() {
            let (x, y) = (self.start.0 as i32 + uv.0, self.start.1 as i32 + uv.1);

            if x < 0 || y < 0 || x >= 140 || y >= 140 {
                continue;
            }

            let facing = *direction as u8;
            let pipe = self.map[x as usize][y as usize];

            if pipe == Pipe::Ground {
                continue;
            }

            let ends = pipe.ends();
            let mirror = Direction::match_dir(facing).opposing_end() as u8;

            if !(mirror & ends).is_power_of_two() {
                continue;
            }

            let exit = mirror ^ ends;

            dir.push_back(((x as u32, y as u32), exit));
        }

        Stream { depth: 1, nodes: dir }
    }

    fn path(&self) -> Path {
        let mut path = Path::new();

        let mut stream = self.start_paths();

        path.insert(stream.depth, stream.nodes.clone().into());

        let mut tock = 1u8;

        while let Some(node) = stream.nodes.pop_front() {
            
            let next_node = self.next(node);

            if path.values().find(|&v| v.contains(&next_node)).is_some() {
                continue;
            }

            path.entry(stream.depth+1).and_modify(|v| v.push(next_node)).or_insert(vec![next_node]);

            if next_node.0 == self.start {
                stream.nodes.pop_front();
                break;
            }

            stream.nodes.push_back(next_node);

            if tock == 2 {
                stream.depth += 1;
                tock = 1;
            }
            else {
                tock += 1;
            }

        }

        path
    }

    fn area(&self, path: Path) -> i32 {

        let mut verts = vec![self.start];

        verts.extend(path.iter().map(|(_, v)| v[0].0));
        
        let mut trail: i32 = 0;
        for (i, xy) in verts.windows(2).enumerate() {
            if i == verts.len()-1 {
                trail += (xy[0].0 as i32 * verts[0].1 as i32) - (xy[0].1 as i32 * verts[0].0 as i32);
            }
            else {
                trail += (xy[0].0 as i32 * xy[1].1 as i32) - (xy[0].1 as i32 * xy[1].0 as i32);
            }
        }

        let area = (trail / 2).abs();

        (area - (path.len() as i32 / 2) + 1).abs()

    }

}

impl Default for PipeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SE,
    SW,
    Start,
    Ground
}

impl Pipe {
    
    fn parse(input: char) -> Self {
        match input {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            '.' => Self::Ground,
            _ => unreachable!()
        }
    }

    fn ends(&self) -> u8 {
        match self {
            Self::Vertical => Direction::N as u8 | Direction::S as u8,
            Self::Horizontal => Direction::E as u8 | Direction::W as u8,
            Self::SE => Direction::E as u8 | Direction::S as u8,
            Self::SW => Direction::W as u8 | Direction::S as u8,
            Self::NE => Direction::N as u8 | Direction::E as u8,
            Self::NW => Direction::N as u8 | Direction::W as u8,
            _ => unreachable!()
        }
    }
}

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
            _ => unreachable!()
        }
    }
}

type Path = BTreeMap<u32, Vec<(Pos, u8)>>;
type Pos = (u32, u32);

pub struct Day10;


impl Solver for Day10 {
    fn title(&self) -> &'static str {
        r"Pipe Maze"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let mut map = PipeMap::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.tile((x as u32, y as u32), c);
            }
        }

        let path = map.path();

        let max_dist = *path.keys().max().unwrap() / 2;

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = max_dist.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();
        let mut map = PipeMap::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.tile((x as u32, y as u32), c);
            }
        }

        let path = map.path();

        let area = map.area(path);

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = area.into();

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

    use super::Day10;

    const MAP: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    const MAP2: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day10.part_a(MAP, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(8));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day10.part_b(MAP2, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(10));
    }

}
