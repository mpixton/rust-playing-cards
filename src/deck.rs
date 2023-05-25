//! Deck of playing cards with associated funationality.
//!
//! At its core, a wrapper around a Vec of [Card]s. Dealing a [Card] removes one
//! from the top of the Deck and returns it. Shuffling the Deck randomizes the
//! order. Building a [Deck] is done by calling configuration methods after a
//! `Deck::new()` call.
//!
//! # Examples
//! ```
//! // Create a new full 52 card deck and shuffle it 7 times
//! let deck = Deck::new().deck_type(DeckType::Full).shuffle(7);
//! assert_eq!(deck.len(), 52);
//! ```
//!
//! # Todo
//! - [ ] Update documentation
//! 

use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Deck<T: DeckState> {
    cards: Vec<Card>,
    extra: T,
}

pub enum DeckType {
    FullFrench,
}

pub trait DeckState {}

struct Start;

struct Building {
    deck_type: Option<DeckType>,
}

struct Shuffling;

struct Finished;

impl DeckState for Start {}
impl DeckState for Building {}
impl DeckState for Shuffling {}
impl DeckState for Finished {}

impl Deck<Start> {
    pub fn custom_new() -> Deck<Building> {
        Deck {
            cards: Vec::new(),
            extra: Building { deck_type: None },
        }
    }

    pub fn default_new() -> Deck<Finished> {
        let deck = Deck {
            cards: Vec::new(),
            extra: Building { deck_type: None },
        };
        deck.deck_type(DeckType::FullFrench).shuffle(7)
    }
}

impl Deck<Building> {
    pub fn deck_type(self, deck_type: DeckType) -> Deck<Shuffling> {
        let deck_size = match deck_type {
            DeckType::FullFrench => 52,
        };

        let mut cards: Vec<Card> = Vec::with_capacity(deck_size);

        for suit in Suit::VALUES.iter() {
            for rank in Rank::VALUES.iter() {
                cards.push(Card::new(*rank, *suit))
            }
        }

        Deck {
            cards,
            extra: Shuffling,
        }
    }
}

impl Deck<Shuffling> {
    /// Shuffles the [Deck] anywhere from 1 to 10 times.
    pub fn shuffle(self, shuffles: usize) -> Deck<Finished> {
        let mut cards = self.cards;
        let mut shuffling = || cards.shuffle(&mut thread_rng());

        match shuffles {
            1..=10 => {
                for _ in 0..=shuffles {
                    shuffling();
                }
            }
            _ => shuffling(),
        }

        let (first, last) = cards.split_at(cards.len() / 2);

        cards = [last, first].concat();

        Deck {
            cards,
            extra: Finished,
        }
    }
}

impl Deck<Finished> {
    pub fn deal_top_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_new_returns_52_card_deck() {
        let deck = Deck::default_new();

        assert_eq!(deck.total_cards(), 52)
    }

    #[test]
    fn custom_new_returns_52_card_deck() {
        let deck = Deck::custom_new()
            .deck_type(DeckType::FullFrench)
            .shuffle(7);

        assert_eq!(deck.total_cards(), 52)
    }
}
