//! All Suits in a French deck of cards.

use std::fmt;

/// Enum of all Suits in a French deck of cards
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suit {
    /// All Suit values for easy iteration
    pub const VALUES: [Suit; 4] = [Self::Hearts, Self::Clubs, Self::Diamonds, Self::Spades];
}

/// Returns a user-friendly string representation of the Suit
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_names_are_correct_when_printed() {
        let expected_names: [String; 4] = [
            "Hearts".into(),
            "Clubs".into(),
            "Diamonds".into(),
            "Spades".into(),
        ];
        let actual_names = Suit::VALUES.map(|e| format!("{}", e));

        assert_eq!(expected_names, actual_names);
    }
}
