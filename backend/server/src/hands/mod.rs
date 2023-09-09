//! Model a [`Hand`] of five [`Card`]s.

mod classify;

pub use classify::classify;

use crate::cards::{Card, Rank, Suit};
use serde::Serialize;
use std::collections::{hash_map::Entry, HashMap, HashSet};

/// Represent which classification a [`Hand`] of five cards has.
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd, Serialize)]
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

/// Represents a hand of five unique [`Card`]s.
///
/// Since the only way to construct a [`Hand`] is via the [`TryFrom`] trait, whose function fails if
/// five unique cards are not given, a [`Hand`] is guaranteed to have five unique [`Cards`].
#[derive(Clone, Serialize, Debug)]
pub struct Hand {
    hand: HashSet<Card>,
}

pub type RankCount = HashMap<Rank, u8>;
pub type SuitCount = HashMap<Suit, u8>;

impl Hand {
    /// Returns an iterator over the five unique cards.
    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.hand.iter()
    }

    /// Gives a count of each of the ranks on hand.
    pub fn count_ranks(&self) -> RankCount {
        let mut ranks = HashMap::new();
        for card in self.cards() {
            match ranks.entry(card.rank) {
                Entry::Occupied(e) => {
                    *e.into_mut() += 1;
                }
                Entry::Vacant(e) => {
                    e.insert(1);
                }
            }
        }
        ranks
    }

    /// Gives a count of each of the suits on hand.
    pub fn count_suits(&self) -> SuitCount {
        let mut suits = HashMap::with_capacity(4);
        for card in self.cards() {
            match suits.entry(card.suit) {
                Entry::Occupied(e) => {
                    *e.into_mut() += 1;
                }
                Entry::Vacant(e) => {
                    e.insert(1);
                }
            }
        }
        suits
    }
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

    /// Attempt to construct a [`Hand`] from a slice of [`Card`]s.
    ///
    /// Fails if the slice does not contain exactly five unique cards.
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
                Card::new(Seven, Spades),
                Card::new(Eight, Spades),
                Card::new(Nine, Spades),
                Card::new(Ten, Spades),
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
                Card::new(Seven, Spades),
                Card::new(Eight, Spades),
                Card::new(Nine, Spades),
                Card::new(Ten, Spades),
                Card::new(Jack, Spades),
                Card::new(Queen, Spades),
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
                Card::new(Seven, Spades),
                Card::new(Eight, Spades),
                Card::new(Nine, Spades),
                Card::new(Ten, Spades),
                Card::new(Ten, Spades),
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
                Card::new(Seven, Spades),
                Card::new(Eight, Spades),
                Card::new(Nine, Spades),
                Card::new(Ten, Spades),
                Card::new(Jack, Spades),
            ]
            .as_slice()
        )
        .is_ok());
    }
}
