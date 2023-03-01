use crate::hands::card::rank::Rank;
use crate::hands::hand_strength::HandStrength;

pub(crate) mod card;
pub(crate) mod hand_strength;

// new struct to Hold a poker hand containing
// a) the individual cards Rank with their occurrence
// b) the index of the hand  the input string
// c) the Strength of the hand
pub(crate) struct Hand {
    cards: Vec<(Rank, i32)>,
    index: usize,
    strength: HandStrength,
}

impl Hand {
    pub(crate) fn new( strength: HandStrength, cards: Vec<(Rank, i32)>, index: usize) -> Self {
        Self {
            index,
            strength,
            cards,
        }
    }
    pub(crate) fn get_index(&self) -> usize {
        self.index
    }
    pub(crate) fn get_strength(&self) -> &HandStrength {
        &self.strength
    }
    pub(crate) fn get_card_ranks(&self) -> Vec<&Rank> {
        self.cards.iter().map(|a| &a.0).collect::<Vec<&Rank>>()
    }
}