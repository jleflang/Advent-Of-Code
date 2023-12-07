use std::{
    time::{Duration, Instant}, 
    collections::HashMap, 
    cmp::Ordering,
};

use schema::Solver;


const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const JOKER_CARDS: [char; 13] = ['J','2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    High,
    Two,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand<'a> {

    cards: &'a str,

    bid: u32,

}

type Hands<'a> = Vec<Hand<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JokerHand<'a> {

    cards: &'a str,

    bid: u32,

}

type JokerHands<'a> = Vec<JokerHand<'a>>;

impl<'a> Hand<'a> {

    fn calculate_hand(&self) -> Kind {

        let mut hand: HashMap<char, u8> = HashMap::new();

        for card in self.cards.chars() {
            hand.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut matches: Vec<u8> = hand.into_values().collect();
        matches.sort();
        matches.reverse();

        match matches[0] {
            5 => Kind::Five,
            4 => Kind::Four,
            3 => {
                    if matches[1] == 2 {
                        return Kind::FullHouse;
                    }

                    Kind::Three
                },
            2 => {
                    if matches[1] == 2 {
                        return Kind::TwoPair;
                    }

                    Kind::Two
                },
            1 => Kind::High,
            _ => unreachable!()
        }

    }
}

impl<'a> JokerHand<'a> {
    
    fn calculate_hand(&self) -> Kind {

        let mut hand: HashMap<char, u8> = HashMap::new();

        for card in self.cards.chars() {
            hand.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        let jokers = hand.remove(&'J').unwrap_or(0u8);

        let mut matches: Vec<u8> = hand.into_values().collect();
        matches.sort();
        matches.reverse();

        match matches.first().unwrap_or(&0u8) + jokers {
            5 => Kind::Five,
            4 => Kind::Four,
            3 => {
                    if matches[1] == 2 {
                        return Kind::FullHouse;
                    }

                    Kind::Three
                },
            2 => {
                    if matches[1] == 2 {
                        return Kind::TwoPair;
                    }

                    Kind::Two
                },
            1 => Kind::High,
            _ => unreachable!()
        }

    }

}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {

        if self.cards == other.cards {
            return Ordering::Equal;
        }

        let mut hand1 = self.cards.chars();
        let mut hand2 = other.cards.chars();

        let mut winner = self.calculate_hand().cmp(&other.calculate_hand());

        while winner == Ordering::Equal {
            let c1 = hand1.next();
            let c2 = hand2.next();

            if c1.is_none() || c2.is_none() {
                break;
            }

            let val1 = CARDS.iter().position(|&c| c == c1.unwrap()).unwrap();
            let val2 = CARDS.iter().position(|&c| c == c2.unwrap()).unwrap();

            winner = val1.cmp(&val2);
        }

        winner
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Default for Hand<'_> {
    fn default() -> Self {
        Self { cards: "", bid: 0 }
    }
}

impl Ord for JokerHand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {

        if self.cards == other.cards {
            return Ordering::Equal;
        }

        let mut hand1 = self.cards.chars();
        let mut hand2 = other.cards.chars();

        let mut winner = self.calculate_hand().cmp(&other.calculate_hand());

        while winner == Ordering::Equal {
            let c1 = hand1.next();
            let c2 = hand2.next();

            if c1.is_none() || c2.is_none() {
                break;
            }

            let val1 = JOKER_CARDS.iter().position(|&c| c == c1.unwrap()).unwrap();
            let val2 = JOKER_CARDS.iter().position(|&c| c == c2.unwrap()).unwrap();

            winner = val1.cmp(&val2);
        }

        winner
    }
}

impl PartialOrd for JokerHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Default for JokerHand<'_> {
    fn default() -> Self {
        Self { cards: "", bid: 0 }
    }
}

pub struct Day07;

impl Solver for Day07 {

    fn title(&self) -> &'static str {
        r"Camel Cards"
    }

    fn part_a(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();
        let mut hands: Hands = Hands::default();

        for l in input.lines() {
            let play: Vec<&str> = l.split_whitespace().collect();

            hands.push(Hand { cards: play[0], bid: play[1].parse::<u32>().unwrap() });
        }

        hands.sort();

        let total: u32 = hands.iter().enumerate().map(|(i, hand)| hand.bid * (i as u32 + 1)).sum();

        let d = ts.elapsed();

        let mut output = out.lock().unwrap();
        *output = total.into();

        Ok(d)

    }

    fn part_b(&self, input: &str, out: std::sync::Arc<std::sync::Mutex<schema::Answer>>) 
            -> Result<Duration, Box<(dyn std::error::Error + Send + Sync)>> {

        let ts = Instant::now();
        let mut hands: JokerHands = JokerHands::default();

        for l in input.lines() {
            let play: Vec<&str> = l.split_whitespace().collect();

            hands.push(JokerHand { cards: play[0], bid: play[1].parse::<u32>().unwrap() });
        }

        hands.sort();

        let total: u32 = hands.iter().enumerate().map(|(i, hand)| hand.bid * (i as u32 + 1)).sum();

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

    use super::Day07;

    const SET1: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part_a() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day07.part_a(SET1, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(6440));
    }

    #[test]
    fn part_b() {
        let out = Arc::new(Mutex::new(Answer::Unimplemented));
        let _ = Day07.part_b(SET1, out.clone());
        assert_eq!(*out.lock().unwrap().deref(), Answer::Number(5905));
    }
}
