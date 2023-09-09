//! Provides the [`Hand`] classification function [`classify`]
//!
//! The remaining functions are used within [`classify`] and are not necessarily correct on their
//! own. E.g., [`is_two_pair`] will return true on a four-of-a-kind hand. Thus, the functions must
//! be called in a specific order, like in [`classify`].

use crate::{
    cards::Rank,
    hands::{Hand, HandCategory, RankCount, SuitCount},
};

use itertools::Itertools;

/// Analyze the five-card [`Hand`] and returns the highest-ranking category possible with it.
pub fn classify(hand: &Hand) -> HandCategory {
    use HandCategory::*;
    let rank_count = hand.count_ranks();
    let suit_count = hand.count_suits();

    let straight = is_straight(&rank_count);
    let flush = is_flush(&suit_count);

    if straight && flush {
        return StraightFlush;
    }

    if is_four_of_a_kind(&rank_count) {
        return FourOfAKind;
    }

    if is_full_house(&rank_count) {
        return FullHouse;
    }

    if flush {
        return Flush;
    }

    if straight {
        return Straight;
    }

    if is_three_of_a_kind(&rank_count) {
        return ThreeOfAKind;
    }

    if is_two_pair(&rank_count) {
        return TwoPair;
    }

    if is_one_pair(&rank_count) {
        return OnePair;
    }

    HighCard
}

fn is_flush(suit_count: &SuitCount) -> bool {
    suit_count.len() == 1
}

fn is_straight(rank_count: &RankCount) -> bool {
    // Exit early if we don't have five different ranks.
    if rank_count.len() != 5 {
        return false;
    }

    let ranks: Vec<_> = rank_count.keys().copied().sorted().collect();

    // Handle special case of Ten through Ace. If "lowest" is Ace and next is Ten, we have a
    // Ten-through-Ace straight, since we have five different ranks.
    if ranks[0] == Rank::Ace && ranks[1] == Rank::Ten {
        return true;
    }

    // Check distance between lowest and highest rank.
    ranks[4].numeric() - ranks[0].numeric() == 4
}

fn is_four_of_a_kind(rank_count: &RankCount) -> bool {
    rank_count.values().any(|&v| v == 4)
}

fn is_full_house(rank_count: &RankCount) -> bool {
    rank_count.len() == 2 && rank_count.values().any(|&v| v == 2 || v == 3)
}

fn is_three_of_a_kind(rank_count: &RankCount) -> bool {
    rank_count.values().any(|&v| v == 3)
}

fn is_two_pair(rank_count: &RankCount) -> bool {
    rank_count.values().filter(|&&v| v == 2).count() == 2
}

fn is_one_pair(rank_count: &RankCount) -> bool {
    rank_count.values().any(|&v| v == 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cards::{Card, Rank, Suit},
        hands::Hand,
    };

    #[test]
    fn royal_straight_flush() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Ten, Suit::Hearts),
                Card::new(Rank::Jack, Suit::Hearts),
                Card::new(Rank::Queen, Suit::Hearts),
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert_eq!(classify(&hand), HandCategory::StraightFlush);
    }

    #[test]
    fn high_card() {
        let hand = Hand::try_from(
            [
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Four, Suit::Spades),
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Diamonds),
                Card::new(Rank::Jack, Suit::Hearts),
            ]
            .as_slice(),
        )
        .unwrap();
        assert_eq!(classify(&hand), HandCategory::HighCard);
    }

    mod flush {
        use super::is_flush;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

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
            assert!(!is_flush(&hand.count_suits()));
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
            assert!(is_flush(&hand.count_suits()));
        }
    }

    mod straight {
        use super::is_straight;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

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
            assert!(!is_straight(&hand.count_ranks()));
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
            assert!(!is_straight(&hand.count_ranks()));
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
            assert!(is_straight(&hand.count_ranks()));
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
            assert!(is_straight(&hand.count_ranks()));
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
            assert!(is_straight(&hand.count_ranks()));
        }
    }

    mod four_of_a_kind {
        use super::is_four_of_a_kind;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

        #[test]
        fn three_of_a_kind_gives_false() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Ace, Suit::Diamonds),
                    Card::new(Rank::Seven, Suit::Clubs),
                    Card::new(Rank::Seven, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(!is_four_of_a_kind(&hand.count_ranks()));
        }

        #[test]
        fn four_of_a_kind_gives_true() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Ace, Suit::Diamonds),
                    Card::new(Rank::Ace, Suit::Clubs),
                    Card::new(Rank::Seven, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(is_four_of_a_kind(&hand.count_ranks()));
        }
    }

    mod full_house {
        use super::is_full_house;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

        #[test]
        fn two_pair_gives_false() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Queen, Suit::Diamonds),
                    Card::new(Rank::Seven, Suit::Clubs),
                    Card::new(Rank::Seven, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(!is_full_house(&hand.count_ranks()));
        }

        #[test]
        fn full_house_gives_true() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Seven, Suit::Diamonds),
                    Card::new(Rank::Seven, Suit::Clubs),
                    Card::new(Rank::Seven, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(is_full_house(&hand.count_ranks()));
        }
    }

    mod three_of_a_kind {
        use super::is_three_of_a_kind;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

        #[test]
        fn a_pair_gives_false() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Jack, Suit::Diamonds),
                    Card::new(Rank::Queen, Suit::Clubs),
                    Card::new(Rank::King, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(!is_three_of_a_kind(&hand.count_ranks()));
        }

        #[test]
        fn three_of_a_kind_gives_true() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Ace, Suit::Diamonds),
                    Card::new(Rank::Queen, Suit::Clubs),
                    Card::new(Rank::King, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(is_three_of_a_kind(&hand.count_ranks()));
        }
    }

    mod two_pair {
        use super::is_two_pair;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

        #[test]
        fn a_pair_gives_false() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Jack, Suit::Diamonds),
                    Card::new(Rank::Queen, Suit::Clubs),
                    Card::new(Rank::King, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(!is_two_pair(&hand.count_ranks()));
        }

        #[test]
        fn two_pair_gives_true() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                    Card::new(Rank::Queen, Suit::Diamonds),
                    Card::new(Rank::Queen, Suit::Clubs),
                    Card::new(Rank::King, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(is_two_pair(&hand.count_ranks()));
        }
    }

    mod one_pair {
        use super::is_one_pair;
        use crate::{
            cards::{Card, Rank, Suit},
            hands::Hand,
        };

        #[test]
        fn high_card_gives_false() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Two, Suit::Spades),
                    Card::new(Rank::Four, Suit::Hearts),
                    Card::new(Rank::Six, Suit::Diamonds),
                    Card::new(Rank::Eight, Suit::Clubs),
                    Card::new(Rank::Ten, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(!is_one_pair(&hand.count_ranks()));
        }

        #[test]
        fn one_pair_gives_true() {
            let hand = Hand::try_from(
                [
                    Card::new(Rank::Two, Suit::Spades),
                    Card::new(Rank::Two, Suit::Hearts),
                    Card::new(Rank::Six, Suit::Diamonds),
                    Card::new(Rank::Eight, Suit::Clubs),
                    Card::new(Rank::Ten, Suit::Hearts),
                ]
                .as_slice(),
            )
            .unwrap();
            assert!(is_one_pair(&hand.count_ranks()));
        }
    }
}
