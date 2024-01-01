/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::collections::HashMap;
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    vec![hands[0]]
}

const CATEGORIES: &[&str] = &["Five of a Kind", "Straight Flush", "Four of a Kind", "Full House", "Flush",
"Straight", "Three of a Kind", "Two Pair", "One Pair", "High card"];

struct Hand<'a> {
    cards: &'a str,
    category: String,
    rank: usize,
}

impl<'a> Hand<'a> {

    fn new(cards: &'a str) -> Self {
        let (category, rank) = Self::annotate_hand(&cards);
        Self { cards, category, rank}
    }

    fn annotate_hand(cards: &'a str) -> (String, usize) {
        // return the category of a card string (e.g. "3S 4S 5D 6H JH")
        let mut rank_hash: HashMap<char, usize> = HashMap::new();
        let mut suit_hash: HashMap<char, usize> = HashMap::new();
        for card in cards.split_whitespace() {
            let rank: char = card.chars().next().unwrap();
            let suit: char = card.chars().last().unwrap();
            *rank_hash.entry(rank).or_insert(0) += 1;
            *suit_hash.entry(suit).or_insert(0) += 1;
        }

        // Check if straight flush:
        if suit_hash.values().any(|&value| value == 5) {
            let mut values: Vec<usize> = rank_hash.values().cloned().collect();
            values.sort();
            let is_continous = values.windows(2).all(|window| window[1] == window[0] + 1);
            if is_continous {
                let category = "Straight Flush";
            } else if  {

            }
        }
    }

    fn is_category(cards: &'a Vec<&str>, category: &str) -> bool {
        match category {
            "Five of a Kind" => {
                todo!("Check if all the cards have the same rank");
                false
            },
            "Straight Flush" => {
                todo!("Check if all the cards are sequential");
                false
            },
            "Four of a Kind" => {
                todo!("Check if all but 1 card have the same rank");
                false
            },
            "Full House" => {
                todo!("Check if the cards are 3 cards of one rank and 2 cards of another rank")
            },
            _ => false,
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        todo!("If hand and rank are equal, equal should return True, else False");
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!("Hand with greater hand is greater, if equal hand, hand with greater rank")
    }
}
