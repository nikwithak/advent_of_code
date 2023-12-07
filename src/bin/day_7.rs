use std::{
    char,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(test)]
mod tests {
    use crate::{parse_input, score_hands};

    #[test]
    fn part_1_sample() {
        let input = parse_input("inputs/day_7_sample.txt", false);
        let result = score_hands(input);
        assert_eq!(result, 6440);
    }
    #[test]
    fn part_1_final() {
        let input = parse_input("inputs/day_7.txt", false);
        let result = score_hands(input);
        assert_eq!(result, 250957639);
    }

    #[test]
    fn part_2_sample() {
        let input = parse_input("inputs/day_7_sample.txt", true);
        let result = score_hands(input);
        assert_eq!(result, 5905);
    }
    #[test]
    fn part_2_final() {
        let input = parse_input("inputs/day_7.txt", true);
        let result = score_hands(input);
        assert_eq!(result, 251515496);
    }
}

fn main() {}

#[derive(Debug)]
struct Hand {
    cards: Vec<Rank>,
    bid: u64,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
enum Rank {
    Joker,
    Number(u8),
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut map: HashMap<&Rank, u64> = HashMap::new();
        for card in &self.cards {
            let val = map.get(&card).unwrap_or(&0);
            map.insert(card, val + 1);
        }

        // PART 2: Account for jokers
        if map.len() > 1 {
            if let Some(jokers) = map.remove(&Rank::Joker) {
                let mut values = map.values_mut().collect::<Vec<_>>();
                values.sort();
                values.last_mut().map(|v| **v += jokers);
            }
        }

        type H = HandType;
        let hand_type = map.values().fold(H::HighCard, |acc, val| match (&acc, val) {
            (H::HighCard, 2) => H::OnePair,
            (H::HighCard, 3) => H::ThreeOfAKind,
            (H::HighCard, 4) => H::FourOfAKind,
            (H::HighCard, 5) => H::FiveOfAKind,
            (H::OnePair, 2) => H::TwoPair,
            (H::OnePair, 3) => H::FullHouse,
            (H::ThreeOfAKind, 2) => H::FullHouse,
            // Nothing else should be possible (don't need to worry about straights / flushes)
            _ => acc,
        });
        hand_type
    }
}

fn score_hands(mut hands: Vec<Hand>) -> u64 {
    hands.sort_by(|lh, rh| {
        let cmp = lh.hand_type().cmp(&rh.hand_type());
        match cmp {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let cmp = lh.cards[i].cmp(&rh.cards[i]);
                    match cmp {
                        std::cmp::Ordering::Equal => continue,
                        _ => return cmp,
                    }
                }
                todo!()
            },
            _ => cmp,
        }
    });

    let mut total = 0 as u64;
    for (i, hand) in hands.iter().enumerate() {
        total += (1 + i as u64) * hand.bid;
    }
    total
}

fn parse_input(
    filename: &str,
    jokers_enabled: bool,
) -> Vec<Hand> {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut hands = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        hands.push(Hand {
            cards: line
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        Rank::Number(c.to_digit(10).unwrap() as u8)
                    },
                    'T' => Rank::Number(10),
                    'A' => Rank::A,
                    'K' => Rank::K,
                    'Q' => Rank::Q,
                    'J' => {
                        if jokers_enabled {
                            Rank::Joker
                        } else {
                            Rank::J
                        }
                    },
                    _ => panic!("invalid card"),
                })
                .collect(),
            bid: line.next().unwrap().parse().unwrap(),
        });
    }
    hands
}
