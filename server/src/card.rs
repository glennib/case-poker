//! Here we model a playing [`Card`] with a [`Rank`] and a [`Suit`].

use serde::Serialize;
use std::str::FromStr;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd, Serialize)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn numeric(self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl TryFrom<char> for Suit {
    type Error = ();
    /// Attempt to return a [Suit] from a char.
    ///
    /// The chars 'r', 's', 'k', 'h' map to Diamonds, Spades, Clubs and Hearts,
    /// respectively. This is following the Norwegian language, i.e., ruter, spar, kløver, hjerter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'r' => Ok(Suit::Diamonds),
            's' => Ok(Suit::Spades),
            'k' => Ok(Suit::Clubs),
            'h' => Ok(Suit::Hearts),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = ();
    /// Attempt to return a [Rank] from a char.
    ///
    /// '1' through '9' map to Ace through Nine. 't' maps to Ten. 'j', 'q', 'k' map to Jack,
    /// Queen, and King.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Rank::Ace),
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            't' => Ok(Rank::Ten),
            'j' => Ok(Rank::Jack),
            'q' => Ok(Rank::Queen),
            'k' => Ok(Rank::King),
            _ => Err(()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InvalidConversion {
    #[error("length of str ({0}) must be 2")]
    Length(usize),
    #[error("{0} is not a valid rank")]
    Rank(char),
    #[error("{0} is not a valid suit")]
    Suit(char),
}

impl FromStr for Card {
    type Err = InvalidConversion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(InvalidConversion::Length(s.len()));
        }
        let mut chars = s.chars();
        let rank = chars.next().expect("we already checked that length is 2");
        let Ok(rank) = Rank::try_from(rank) else {
            return Err(InvalidConversion::Rank(rank));
        };
        let suit = chars.next().expect("we already checked that length is 2");
        let Ok(suit) = Suit::try_from(suit) else {
            return Err(InvalidConversion::Suit(suit));
        };
        Ok(Self::new(rank, suit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_card_conversions_work() {
        assert_eq!(Card::new(Rank::Four, Suit::Diamonds), "4r".parse().unwrap());
        assert_eq!(Card::new(Rank::Jack, Suit::Spades), "js".parse().unwrap());
        assert_eq!(Card::new(Rank::Ten, Suit::Clubs), "tk".parse().unwrap());
    }

    #[test]
    fn invalid_length_yields_error() {
        assert!("".parse::<Card>().is_err());
        assert!("tkk".parse::<Card>().is_err());
        assert!("jjs".parse::<Card>().is_err());
    }

    #[test]
    fn invalid_rank_yields_error() {
        assert!("0k".parse::<Card>().is_err());
    }

    #[test]
    fn invalid_suit_yields_error() {
        assert!("1p".parse::<Card>().is_err());
    }
}
