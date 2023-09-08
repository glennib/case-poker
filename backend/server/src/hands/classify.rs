use crate::hands::Hand;

use crate::cards::Rank;
use itertools::Itertools;

fn is_flush(hand: &Hand) -> bool {
    let mut cards = hand.cards();
    let head = cards.next().expect("a hand has five cards");
    cards.all(|card| card.suit == head.suit)
}

#[cfg(test)]
mod test_flush {
    use super::is_flush;
    use crate::cards::{Card, Rank, Suit};
    use crate::hands::Hand;

    #[test]
    fn one_unequal_gives_false() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Five, Suit::Spades),
                Card::new(Rank::Seven, Suit::Spades),
                Card::new(Rank::Nine, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(!is_flush(&hand));
    }

    #[test]
    fn flush_gives_true() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Five, Suit::Spades),
                Card::new(Rank::Seven, Suit::Spades),
                Card::new(Rank::Nine, Suit::Spades),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(is_flush(&hand));
    }
}

fn is_straight(hand: &Hand) -> bool {
    // Keeping track of whether the first card is Ten, and whether we're at the final (n == 3) card
    // is done to allow wrapping around for the case of Ten-Ace straight
    // TODO: This doesn't work. Since we sort, Ace is always first.
    let mut ranks = hand.cards().map(|c| c.rank).sorted();
    let mut previous = ranks.next().expect("a hand has five cards");
    let first_is_ten = previous == Rank::Ten;
    for (n, current) in ranks.enumerate() {
        if first_is_ten && n == 3 && current == Rank::Ace {
            return true;
        }
        if current != previous.next() {
            return false;
        }
        previous = current;
    }
    true
}

#[cfg(test)]
mod test_straight {
    use super::is_straight;
    use crate::cards::{Card, Rank, Suit};
    use crate::hands::Hand;

    #[test]
    fn split_in_middle_gives_false() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Four, Suit::Hearts),
                Card::new(Rank::Six, Suit::Diamonds),
                Card::new(Rank::Seven, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(!is_straight(&hand));
    }

    #[test]
    fn wrap_around_gives_false() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Jack, Suit::Spades),
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Diamonds),
                Card::new(Rank::Two, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(!is_straight(&hand));
    }

    #[test]
    fn straight_in_middle_gives_true() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Five, Suit::Spades),
                Card::new(Rank::Six, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Hearts),
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::Nine, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(is_straight(&hand));
    }

    #[test]
    fn ace_through_five_gives_true() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Hearts),
                Card::new(Rank::Four, Suit::Diamonds),
                Card::new(Rank::Five, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(is_straight(&hand));
    }

    #[test]
    fn ten_through_ace_gives_true() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Ten, Suit::Spades),
                Card::new(Rank::Jack, Suit::Clubs),
                Card::new(Rank::Queen, Suit::Hearts),
                Card::new(Rank::King, Suit::Diamonds),
                Card::new(Rank::Ace, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert!(is_straight(&hand));
    }
}

fn is_four_of_a_kind(hand: &Hand) -> bool {
    let mut ranks = hand.cards().map(|c| c.rank).sorted();
    let first = ranks.next().expect("a hand has five cards");
    let second = ranks.next().expect("a hand has five cards");
    if first == second {
        return ranks.take(2).all(|r| r == first);
    }
    ranks.take(3).all(|r| r == second)
}
