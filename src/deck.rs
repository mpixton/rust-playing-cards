//! Deck of playing cards with associated funationality.
//!
//! At its core, a wrapper around a Vec of [Card]s. Dealing a [Card] removes one
//! from the top of the Deck and returns it. Shuffling the Deck randomizes the
//! order. Building a [Deck] is done by calling configuration methods after a
//! `Deck::custom_new()` call to customize the [Deck] or `Deck::default_new()` to
//! build a standard French deck.
//! //!
//! # Examples
//! ```
//! // Create a new full 52 card deck and shuffle it 7 times
//! let deck = Deck::custom_new().deck_type(DeckType::FullFrench).shuffle(7);
//! assert_eq!(deck.total_cards(), 52);
//! ```
//!
//! # Todo
//! - [ ] Update documentation
//!

use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// A deck of playing cards.
pub struct Deck<T: DeckState> {
    cards: VecDeque<Card>,
    state: PhantomData<T>,
}

/// Types of [Deck]s.
pub enum DeckType {
    FullFrench,
}

/// TypeState trait to control valid states of the [Deck].
pub trait DeckState {}

struct Start;

struct Building;

struct Shuffling;

struct Finished;

impl DeckState for Start {}
impl DeckState for Building {}
impl DeckState for Shuffling {}
impl DeckState for Finished {}

impl Deck<Start> {
    /// Begin creation of a custom [Deck].
    pub fn custom_new() -> Deck<Building> {
        Deck {
            cards: VecDeque::new(),
            state: PhantomData,
        }
    }

    /// Create a default playing card [Deck], shuffled 7 times.
    pub fn default_new() -> Deck<Finished> {
        let deck = Deck {
            cards: VecDeque::new(),
            state: PhantomData,
        };

        deck.deck_type(DeckType::FullFrench).shuffle(7)
    }
}

impl Deck<Building> {
    /// Configure the [Deck] as a provided custom [DeckType].
    pub fn deck_type(self, deck_type: DeckType) -> Deck<Shuffling> {
        let deck_size = match deck_type {
            DeckType::FullFrench => 52,
        };

        let mut cards: VecDeque<Card> = VecDeque::with_capacity(deck_size);

        for suit in Suit::VALUES.iter() {
            for rank in Rank::VALUES.iter() {
                cards.push_back(Card::new(*rank, *suit))
            }
        }

        Deck {
            cards,
            state: PhantomData,
        }
    }
}

impl Deck<Shuffling> {
    /// Shuffles the [Deck] anywhere from 1 to 10 times.
    pub fn shuffle(self, shuffles: usize) -> Deck<Finished> {
        let mut cards = self.cards;
        let mut shuffling = || cards.make_contiguous().shuffle(&mut thread_rng());

        match shuffles {
            1..=10 => {
                for _ in 0..=shuffles {
                    shuffling();
                }
            }
            _ => shuffling(),
        }

        let halfway = cards.len() / 2;

        let (first, last) = cards.make_contiguous().split_at(halfway);

        let cards: VecDeque<Card> = [last, first].concat().into();

        Deck {
            cards,
            state: PhantomData,
        }
    }
}

impl Deck<Finished> {
    /// Deals the top [Card] from the [Deck].
    pub fn deal_top_card(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    /// Deals the bottom [Card] from the [Deck].
    pub fn deal_bottom_card(&mut self) -> Option<Card> {
        self.cards.pop_back()
    }

    /// Get the all [Card]s left in the [Deck].
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
