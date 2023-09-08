use crate::cards::{Card, Rank, Suit};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

pub struct Hand {
    hand: HashSet<Card>,
}

#[derive(thiserror::Error, Debug)]
pub enum HandConstructionError {
    #[error("number of cards in hand ({0}) must be 5")]
    Length(usize),
    #[error("got {0} unique cards, need 5")]
    Uniqueness(usize),
}

impl TryFrom<&[Card]> for Hand {
    type Error = HandConstructionError;

    fn try_from(value: &[Card]) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err(HandConstructionError::Length(value.len()));
        }
        let hand = HashSet::from_iter(value.iter().copied());
        if hand.len() != 5 {
            return Err(HandConstructionError::Uniqueness(hand.len()));
        }
        Ok(Self { hand })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn too_few_cards_fail() {
        use Rank::*;
        use Suit::*;
        assert!(Hand::try_from(
            [
                Card(Seven, Spades),
                Card(Eight, Spades),
                Card(Nine, Spades),
                Card(Ten, Spades),
            ]
            .as_slice()
        )
        .is_err());
    }
    #[test]
    fn too_many_cards_fail() {
        use Rank::*;
        use Suit::*;
        assert!(Hand::try_from(
            [
                Card(Seven, Spades),
                Card(Eight, Spades),
                Card(Nine, Spades),
                Card(Ten, Spades),
                Card(Jack, Spades),
                Card(Queen, Spades),
            ]
            .as_slice()
        )
        .is_err());
    }
    #[test]
    fn non_unique_cards_fail() {
        use Rank::*;
        use Suit::*;
        assert!(Hand::try_from(
            [
                Card(Seven, Spades),
                Card(Eight, Spades),
                Card(Nine, Spades),
                Card(Ten, Spades),
                Card(Ten, Spades),
            ]
            .as_slice()
        )
        .is_err());
    }
    #[test]
    fn five_unique_cards_succeed() {
        use Rank::*;
        use Suit::*;
        assert!(Hand::try_from(
            [
                Card(Seven, Spades),
                Card(Eight, Spades),
                Card(Nine, Spades),
                Card(Ten, Spades),
                Card(Jack, Spades),
            ]
            .as_slice()
        )
        .is_ok());
    }
}
