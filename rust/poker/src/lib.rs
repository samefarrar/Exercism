/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<Hand> = hands.iter().map(|h| Hand::new(h)).collect();
    hands.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut winners = Vec::new();
    if let Some(first_hand) = hands.first() {
        for hand in hands.iter() {
            if hand == first_hand {
                winners.push(hand.hand_string)
            } else {
                break;
            }
        }
    }
    winners
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Category {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

use std::cmp::Ordering;

#[derive(Debug, Eq, Clone, Copy)]
enum Rank {
    Number(u8),
    J,
    Q,
    K,
    A,
}

impl Rank {
    fn value(&self) -> u8 {
        match self {
            Rank::Number(val) => *val,
            Rank::J => 11,
            Rank::Q => 12,
            Rank::K => 13,
            Rank::A => 14,
        }
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, Clone)]
struct Hand<'a> {
    cards: [Card; 5],
    category: Category,
    ranks: [Rank; 5],
    hand_string: &'a str,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str) -> Self {
        let hand_string = cards;
        let cards: Vec<Card> = cards.split(" ").map(|s| Card::new(s)).collect();
        let cards_array: [Card; 5] = cards.try_into().unwrap();
        let (category, ranks) = Self::annotate_hand(&cards_array);
        Self {
            cards: cards_array,
            category,
            ranks,
            hand_string,
        }
    }

    fn annotate_hand(cards: &[Card; 5]) -> (Category, [Rank; 5]) {
        let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
        ranks.sort_by(|a, b| b.cmp(a));
        let ranks_array: [Rank; 5] = ranks.try_into().unwrap();
        if is_five_of_a_kind(cards) {
            return (Category::FiveOfAKind, ranks_array);
        } else if is_straight_flush(cards) {
            return (Category::StraightFlush, ranks_array);
        } else if is_four_of_a_kind(cards) {
            return (Category::FourOfAKind, ranks_array);
        } else if is_full_house(cards) {
            return (Category::FullHouse, ranks_array);
        } else if is_flush(cards) {
            return (Category::Flush, ranks_array);
        } else if is_straight(cards) {
            return (Category::Straight, ranks_array);
        } else if is_three_of_a_kind(cards) {
            return (Category::ThreeOfAKind, ranks_array);
        } else if is_two_pairs(cards) {
            return (Category::TwoPairs, ranks_array);
        } else if is_one_pair(cards) {
            return (Category::OnePair, ranks_array);
        } else {
            let category = Category::HighCard;
            return (category, ranks_array);
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    suit: char,
    rank: Rank,
}

impl Card {
    fn new(card: &str) -> Self {
        let rank = match &card[0..card.len() - 1] {
            "2" => Rank::Number(2),
            "3" => Rank::Number(3),
            "4" => Rank::Number(4),
            "5" => Rank::Number(5),
            "6" => Rank::Number(6),
            "7" => Rank::Number(7),
            "8" => Rank::Number(8),
            "9" => Rank::Number(9),
            "10" => Rank::Number(10),
            "J" => Rank::J,
            "Q" => Rank::Q,
            "K" => Rank::K,
            "A" => Rank::A,
            &_ => panic!("Invalid card rank"),
        };
        let suit = card.chars().last().unwrap();
        Self { rank, suit }
    }
}

fn is_five_of_a_kind(cards: &[Card; 5]) -> bool {
    return false;
}

fn is_straight_flush(cards: &[Card; 5]) -> bool {
    let first_card_suit = cards.first().unwrap().suit;
    if !cards.iter().all(|c| c.suit == first_card_suit) {
        return false;
    }
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by_key(|c| c.rank);
    for window in sorted_cards.windows(2) {
        if let [first_card, second_card] = window {
            if first_card.suit != second_card.suit
                || first_card.rank.value() != second_card.rank.value() + 1
            {
                return false;
            }
        } else {
            panic!("Unexpected window length.")
        }
    }
    return true;
}

fn is_four_of_a_kind(cards: &[Card; 5]) -> bool {
    false
}

fn is_full_house(cards: &[Card; 5]) -> bool {
    false
}

fn is_flush(cards: &[Card; 5]) -> bool {
    false
}

fn is_straight(cards: &[Card; 5]) -> bool {
    false
}

fn is_three_of_a_kind(cards: &[Card; 5]) -> bool {
    false
}

fn is_two_pairs(cards: &[Card; 5]) -> bool {
    false
}

fn is_one_pair(cards: &[Card; 5]) -> bool {
    false
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.ranks == other.ranks
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.category == other.category {
            self.ranks.partial_cmp(&other.ranks)
        } else {
            self.category.partial_cmp(&other.category)
        }
    }
}
