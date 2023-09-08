#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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
    /// The chars 'r', 's', 'k', 'h' map to [Diamonds], [Spades], [Clubs] and [Hearts],
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
    /// '1' through '9' map to [Ace] through [Nine]. 't' maps to [Ten]. 'j', 'q', 'k' map to [Jack],
    /// [Queen], and [King].
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

impl TryFrom<&str> for Card {
    type Error = InvalidConversion;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(InvalidConversion::Length(value.len()));
        }
        let mut chars = value.chars();
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
        assert_eq!(
            Card::try_from("4r").unwrap(),
            Card::new(Rank::Four, Suit::Diamonds,)
        );
        assert_eq!(
            Card::try_from("js").unwrap(),
            Card::new(Rank::Jack, Suit::Spades,)
        );
        assert_eq!(
            Card::try_from("tk").unwrap(),
            Card::new(Rank::Ten, Suit::Clubs,)
        );
    }
    #[test]
    fn invalid_length_yields_error() {
        assert!(Card::try_from("").is_err());
        assert!(Card::try_from("tkk").is_err());
        assert!(Card::try_from("jjs").is_err());
    }
    #[test]
    fn invalid_rank_yields_error() {
        assert!(Card::try_from("0k").is_err());
    }
    #[test]
    fn invalid_suit_yields_error() {
        assert!(Card::try_from("1p").is_err());
    }
}