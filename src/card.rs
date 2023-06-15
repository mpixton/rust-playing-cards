//! A playing card composed of a [Rank] and [Suit] for playing card games.
//!
//! The [Rank] determines where the Card sorts among other Cards within its
//! [Suit]. The [Suit] determines which grouping of Cards the Card belongs to.

use crate::rank::Rank;
use crate::suit::Suit;
use std::fmt;

/// A Card, representing a traditional Card from a French deck of playing cards.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Creates a new Card with the given [Rank] and [Suit].
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    /// Get the [Rank] of the [Card].
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// Access the [Suit] of the Card.
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

mod tests {
    use super::*;

    #[allow(dead_code)]
    fn setup() -> Card {
        Card::new(Rank::Ace, Suit::Hearts)
    }

    #[test]
    fn suit_returns_card_suit() {
        let card = setup();
        assert_eq!(card.suit(), Suit::Hearts);
    }

    #[test]
    fn rank_returns_card_rank() {
        let card = setup();
        assert_eq!(card.rank(), Rank::Ace);
    }
}
