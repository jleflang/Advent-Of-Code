use std::time::{Duration, Instant};
use ahash::AHashSet;

use schema::Solver;

#[derive(Debug, Default)]
struct Card {
    winning_nums: AHashSet<u32>,
    scratch_nums: AHashSet<u32>,

    win_nums: Vec<u32>,
}

impl Card {
    
    fn find_winnings(&mut self) {
        for snum in self.scratch_nums.intersection(&self.winning_nums) {
            self.win_nums.push(*snum);
        }
    }

    fn total(&self) -> u32 {
        1 << (self.win_nums.len().saturating_sub(1))
    }
}

pub struct Day04;

impl Solver for Day04 {
    fn title(&self) -> &'static str {
        r"Scratchcards"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let mut cards: Vec<Card> = Vec::new();

        let ts = Instant::now();
        for line in input.lines() {
            let (_, game) = line.split_once(':').unwrap();

            let (win, scratch) = game.trim().split_once('|').unwrap();

            let mut dummy = Card::default();

            for w in win.split_whitespace() {
                dummy.winning_nums.insert(w.parse::<u32>().unwrap());
            }

            for s in scratch.split_whitespace() {
                dummy.scratch_nums.insert(s.parse::<u32>().unwrap());
            }

            dummy.find_winnings();
            cards.push(dummy);
        }

        let mut total = 0;

        for c in cards.iter().filter(|x| !x.win_nums.is_empty()) {
            total += c.total();
        }

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {
        let mut cards: Vec<Card> = Vec::new();

        let ts = Instant::now();
        for line in input.lines() {
            let (_, game) = line.split_once(':').unwrap();

            let (win, scratch) = game.trim().split_once('|').unwrap();

            let mut dummy = Card::default();

            for w in win.split_whitespace() {
                dummy.winning_nums.insert(w.parse::<u32>().unwrap());
            }

            for s in scratch.split_whitespace() {
                dummy.scratch_nums.insert(s.parse::<u32>().unwrap());
            }

            dummy.find_winnings();
            cards.push(dummy);
        }

        let mut total = 0;
        let mut q = (0..cards.len()).collect::<Vec<usize>>();

        while let Some(i) = q.pop() {
            total += 1;

            let c = &cards[i];
            if !c.win_nums.is_empty() {
                for w in 0..c.win_nums.len() {
                    q.push(w + i + 1);
                }
            }

        }
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

    use super::Day04;

    const CARDS: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day04.part_a(CARDS, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(13));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day04.part_b(CARDS, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(30));
    }
}
