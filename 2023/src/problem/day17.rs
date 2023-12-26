use std::{time::{Duration, Instant}, hash::Hash};
use ahash::{HashMap, HashMapExt};
use orx_priority_queue::*;


use schema::Solver;


#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Direction {
    N = 0b0001,
    E = 0b0010,
    W = 0b0100,
    S = 0b1000
}

impl Direction {
    fn reverse_dir(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::W => Self::E,
            Self::S => Self::N
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

impl Default for Direction {
    fn default() -> Self {
        Self::E
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    pos: Pos,
    direction: Option<Direction>,
    steps: usize
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.direction.hash(state);
    }
}

#[derive(Debug, Default, Clone)]
struct Walker {
    start: Pos,
    goal: Pos,
    total_loss: usize,
}

#[inline]
fn bounds(next_node: Pos, puzzle: &Puzzle) -> bool {
    next_node.0 < 0 || next_node.0 >= puzzle.len() as i32 ||
        next_node.1 < 0 || next_node.1 >= puzzle[next_node.0 as usize].len() as i32 
}

impl Walker {
    fn set_pos(&mut self, xy: Pos) -> &mut Self {
        self.start = xy;

        self
    }

    fn set_goal(&mut self, goal: Pos) {
        self.goal = goal;
    }

    fn find_neighbors<const MAX: usize>(&self, node: Node, puzzle: &Puzzle) -> Vec<(Node, usize)> {

        [Direction::N, Direction::E, Direction::W, Direction::S].iter().flat_map(|direction| {
            let mut out: Vec<(Node, usize)> = Vec::with_capacity(4);
            let p = direction.move_dir(node.pos);

            if let Some(n_dir) = node.direction {
                if !bounds(p, puzzle) && *direction != n_dir.reverse_dir() {
                    let dist = match n_dir == *direction {
                        true => node.steps + 1,
                        false => 1
                    };

                    match dist <= MAX {
                        true => {
                            out.push((Node {pos: p, direction: Some(*direction), steps: dist}, 
                                            puzzle[p.0 as usize][p.1 as usize] as usize ))
                        },
                        false => ()
                    }
                }
            } else if *direction == Direction::S || *direction == Direction::E {
                out.push((Node {pos: p, direction: Some(*direction), steps: 1}, 
                                puzzle[p.0 as usize][p.1 as usize] as usize ))
            }

            out

        }).collect()

    }

    fn forward_neighbors(&self, node: Node, puzzle: &Puzzle) -> Vec<(Node, usize)> {
        let p = node.direction.unwrap().move_dir(node.pos);

        match !bounds(p, puzzle) {
            true => {
                        let next_node = Node {
                            pos: p, 
                            direction: node.direction,
                            steps: node.steps + 1
                        };
                        vec![(next_node, puzzle[p.0 as usize][p.1 as usize] as usize)]
                    },
            false => Vec::with_capacity(0)
        }

    }

    fn walk<const MIN: usize, const MAX: usize>(&mut self, 
                                                mut unvisited: impl PriorityQueueDecKey<Node, usize>, puzzle: &Puzzle) 
    {

        let mut seen: HashMap<Node, usize> = HashMap::new();

        seen.insert(Node {pos: self.start, direction: None, steps: 0}, 0);

        unvisited.clear();
        unvisited.push(Node {pos: self.start, direction: None, steps: 0}, 0);

        while let Some((node, g)) = unvisited.pop() {

            if node.pos == self.goal {
                self.total_loss = g;
                break;
            }

            let neighbors = match node.steps >= MIN || node.direction.is_none() {
                true => self.find_neighbors::<MAX>(node, puzzle),
                false => self.forward_neighbors(node, puzzle)
            };

            for neighbor in neighbors.iter() {

                let next_node = neighbor.0;

                if let Some(&g_t) = seen.get(&node) {
                    let next_pos = next_node.pos;
                    let next_loss = g_t + puzzle[next_pos.0 as usize][next_pos.1 as usize] as usize;
                    
                    if seen.get(&next_node).map_or(true, |&g_n| next_loss < g_n) {
                        seen.insert(next_node, next_loss);
                        unvisited.try_decrease_key_or_push(&next_node, next_loss);
                    }
                }
            }

        }

    }

}

type Puzzle = Vec<Vec<u8>>;
type Pos = (i32, i32);

pub struct Day17;


impl Solver for Day17 {
    fn title(&self) -> &'static str {
        r"Clumsy Crucible"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        
        let ts = Instant::now();

        let mut puzzle = Puzzle::default();

        for line in input.lines() {
            puzzle.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
        }

        let unvisited: QuarternaryHeapWithMap<Node, usize> = QuarternaryHeapWithMap::default();

        let mut walker = Walker::default();
        walker.set_pos((0, 0));
        walker.set_goal((puzzle.len() as i32 - 1, puzzle[0].len() as i32 - 1));

        walker.walk::<1, 3>(unvisited, &puzzle);

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = walker.total_loss.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();

        let mut puzzle = Puzzle::default();

        for line in input.lines() {
            puzzle.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
        }

        let unvisited: QuarternaryHeapWithMap<Node, usize> = QuarternaryHeapWithMap::default();

        let mut walker = Walker::default();
        walker.set_pos((0, 0));
        walker.set_goal((puzzle.len() as i32 - 1, puzzle[0].len() as i32 - 1));

        walker.walk::<4, 10>(unvisited, &puzzle);

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = walker.total_loss.into();

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

    use super::Day17;

    const PUZZLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day17.part_a(PUZZLE, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(102));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day17.part_b(PUZZLE, out.clone());
        assert_eq!(*out.lock().unwrap(), Answer::Number(94));
    }

}
