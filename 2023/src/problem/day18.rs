use std::time::{Duration, Instant};


use schema::Solver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    U(u32),
    D(u32),
    R(u32),
    L(u32)
}

impl From<[u8; 2]> for Direction {
    fn from(value: [u8; 2]) -> Self {
        match value[0] {
            b'U' => Self::U(value[1].into()),
            b'D' => Self::D(value[1].into()),
            b'R' => Self::R(value[1].into()),
            b'L' => Self::L(value[1].into()),
            _    => unreachable!()
        }
    }
}

impl Direction {
    fn unwrap(&self) -> u32 {
        match self {
            Self::U(n) => *n,
            Self::D(n) => *n,
            Self::L(n) => *n,
            Self::R(n) => *n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PlanItem {
    direction: Direction,
    coords: MapRange,
}

impl PlanItem {
    fn build(input: &str, is_radix: bool) -> Self {
        let t = input.split_whitespace().collect::<Vec<_>>();

        let (dir, c) = t.split_at(2);
        let mut direction = Direction::from([dir[0].as_bytes()[0], dir[1].parse::<u8>().expect("Bad value")]);
        let color: &str = c.first().unwrap().trim_matches(|c| c == '(' || c == ')');

        if is_radix {
            direction = Self::radix_coords(color);
        }

        Self { direction, coords: ((0, 0), (0, 0)) }
    }

    #[allow(unused_assignments)]
    fn compute_coords(&mut self, x: &mut isize, y: &mut isize) {
        
        let (mut x_min, mut y_min) = (0isize, 0isize);
        let (mut x_max, mut y_max) = (0isize, 0isize);

        match self.direction {
            Direction::L(i) | Direction::R(i) => {
                let dir;
                if let Direction::L(_) = self.direction {
                    dir = -(i as isize);
                    x_min = x.checked_sub_unsigned(i as usize).unwrap();
                    x_max = *x;
                    x_min
                }
                else {
                    dir = i as isize;
                    x_max = x.checked_add_unsigned(i as usize).unwrap();
                    x_min = *x;
                    x_max
                };
                *x += dir;
            },
            Direction::U(i) | Direction::D(i) => {
                let dir;
                if let Direction::U(_) = self.direction {
                    dir = -(i as isize);
                    y_min = y.checked_sub_unsigned(i as usize).unwrap();
                    y_max = *y;
                }
                else {
                    dir = i as isize;
                    y_max = y.checked_add_unsigned(i as usize).unwrap();
                    y_min = *y;
                }
                *y += dir;
            }
        }

        self.coords = ((x_min, x_max), (y_min, y_max));

    }

    fn radix_coords(color: &str) -> Direction {
        let tmp = color.strip_prefix("#").unwrap();
        
        let (ctmp, cdir) = tmp.split_at(5);

        let num = u32::from_str_radix(ctmp, 16).unwrap();

        match cdir {
            "0" => Direction::R(num),
            "1" => Direction::D(num),
            "2" => Direction::L(num),
            "3" => Direction::U(num),
            _   => unreachable!()
        }

    }

    #[inline]
    fn get_norm_coords(&self, r: MapRange) -> MapRange {
        let raw_coords = self.coords;

        ((raw_coords.0.0 + r.0.0.abs(), raw_coords.0.1 + r.0.0.abs()), 
         (raw_coords.1.0 + r.1.0.abs(), raw_coords.1.1 + r.1.0.abs()))
    }
}

type MapRange = ((isize, isize), (isize, isize));



fn map_limits(instrs: &[PlanItem]) -> MapRange {
    let (mut x_min, mut x_max) = (isize::MAX, isize::MIN);
    let (mut y_min, mut y_max) = (isize::MAX, isize::MIN);
    let (mut x, mut y) = (0isize, 0isize);

    for instr in instrs.iter() {
        match instr.direction {
            Direction::U(i) => y = y.checked_sub_unsigned(i as usize).unwrap(),
            Direction::D(i) => y = y.checked_add_unsigned(i as usize).unwrap(),
            Direction::L(i) => x = x.checked_sub_unsigned(i as usize).unwrap(),
            Direction::R(i) => x = x.checked_add_unsigned(i as usize).unwrap()
        }

        x_min = x_min.min(x);
        y_min = y_min.min(y);
        x_max = x_max.max(x);
        y_max = y_max.max(y);

    }

    ((x_min, x_max), (y_min, y_max))
}

struct Map {
    verts: Vec<(usize, usize)>,
    limits: MapRange,
    circum: isize
}

impl Map {
    fn new(map_range: MapRange, len: usize) -> Self {
        Self { limits: map_range, verts: Vec::with_capacity(len), circum: 0 }
    }

    fn area(&mut self) -> usize {
        let mut trail: isize = 0;
        for xy in self.verts.windows(2) {
            trail += (xy[0].0 as isize * xy[1].1 as isize) - (xy[0].1 as isize * xy[1].0 as isize);
        }

        let area = (trail / 2).abs();

        (area as isize + (self.circum / 2) + 1).abs() as usize
    }

    #[inline(always)]
    fn add_circumference(&mut self, d: Direction) {
        self.circum += d.unwrap() as isize;
    }

}


pub struct Day18;


impl Solver for Day18 {
    fn title(&self) -> &'static str {
        r"Lavaduct Lagoon"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let ts = Instant::now();

        let lines = input.lines();
        let mut instrs = Vec::with_capacity(lines.clone().count());

        for line in lines {
            instrs.push(PlanItem::build(line, false));
        }

        let map_range = map_limits(&instrs);
        let mut map = Map::new(map_range, instrs.len());

        let (mut x, mut y) = (0isize, 0isize);

        map.verts.reserve(instrs.len());
        map.verts.push((0, 0));

        instrs.iter_mut().for_each(|instr| {
            instr.compute_coords(&mut x, &mut y);
            map.add_circumference(instr.direction);
            map.verts.push((x as usize, y as usize));
            let _tmp = instr.get_norm_coords(map.limits);
        });

        let total = map.area();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)
    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let ts = Instant::now();

        let lines = input.lines();
        let mut instrs = Vec::with_capacity(lines.clone().count());

        for line in lines {
            instrs.push(PlanItem::build(line, true));
        }

        let map_range = map_limits(&instrs);
        let mut map = Map::new(map_range, instrs.len());

        let (mut x, mut y) = (0isize, 0isize);

        map.verts.push((0, 0));

        instrs.iter_mut().for_each(|instr| {
            instr.compute_coords(&mut x, &mut y);
            map.add_circumference(instr.direction);
            map.verts.push((x as usize, y as usize));
            let _tmp = instr.get_norm_coords(map.limits);
        });

        let total = map.area();

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
    use std::{sync::Arc, sync::Mutex};

    use schema::*;

    use indoc::indoc;

    use super::Day18;

    const GRID: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day18.part_a(GRID, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(62));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day18.part_b(GRID, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(952408144115));
    }

}
