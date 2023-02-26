use crate::hands::card::Rank;
use crate::hands::hand_strength::HandStrength;

pub(crate) mod card;
pub(crate) mod hand_strength;

pub(crate) struct Hand{
    is_flush:bool,
    cards:Vec<(Rank, i32)>,
    index:usize,
    strength: HandStrength
}

impl Hand {
    pub(crate) fn new(is_flush:bool, cards:Vec<(Rank, i32)>, index:usize, strength:HandStrength) ->Self{
        Self{
            is_flush,
            index,
            strength,
            cards
        }

    }
    pub(crate) fn get_index(&self)->&usize{
        &self.index
    }
    pub(crate) fn get_strength(&self)->&HandStrength{
        &self.strength
    }
    pub(crate) fn is_flush(&self)->&bool{
        &self.is_flush
    }
    pub(crate) fn get_cards(&self)->&Vec<(Rank, i32)>{
        &self.cards
    }
}