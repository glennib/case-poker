use itertools::Itertools;
use lazy_static::lazy_static;
use crate::cards::{Card, Rank, Suit};
use crate::hands::Hand;

/// Draw a [`Hand`] of five unique [`Card`]s.
pub fn draw_hand() -> Hand {
    use rand::seq::SliceRandom;

    let mut rng = &mut rand::thread_rng();
    let hand: Vec<_> = DECK.as_slice().choose_multiple(&mut rng, 5).copied().collect();
    Hand::try_from(hand.as_slice()).expect("we gave five unique cards")
}

const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

const ALL_RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

lazy_static! {
    static ref DECK: Vec<Card> = {
        use itertools::Itertools;
        ALL_SUITS
            .iter()
            .copied()
            .cartesian_product(ALL_RANKS.iter().copied())
            .map(|(suit, rank)| Card::new(rank, suit))
            .collect()
    };
}

#[cfg(test)]
mod tests {
    use crate::deck::draw_hand;

    #[test]
    pub fn can_draw() {
        for _ in 0..1000 {
            draw_hand();
        }
    }
}
