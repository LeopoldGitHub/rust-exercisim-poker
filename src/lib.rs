mod hands;

use std::collections::{HashMap, HashSet};
use crate::hands::card::{Card, Rank, Suit};
use crate::hands::Hand;
use crate::hands::hand_strength::HandStrength;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands_copy = hands.clone();
    // creates a iterator with a yielded counter over the given array of hands
    hands_copy
        .iter()
        .enumerate()
        .for_each(|hand| {
            // new hashmap to check for repeating Ranks and getting the amount.
            let mut ranks_hash = HashMap::new();
            // new hashset to check if all cards have the same suit.
            let mut suit_hash = HashSet::new();

            // splits the tuple the iterator provides into 2 variables
            let (index, cards) = hand;
            // splits our provided Hand into individual Cards
            cards
                .split(" ")
                .for_each(|card| {

                    ranks_hash
                        // trys to get the value to the given Card Rank
                        .entry(Rank::new(&card))
                        // then increases its occurrence by 1
                        .and_modify(|n| *n += 1)
                        // or if the key is not found adds it with a value of 1
                        .or_insert(1);
                    //
                    suit_hash
                        .insert(Suit::new(&card));
                }
                );
            let mut ranks_vec:Vec<(Rank,i32)>=ranks_hash.into_iter().collect();
            ranks_vec.sort_by(|a,b|b.1.cmp(&a.1))

            // hands_vec.push(Hand::new(is_flush(&card_values),/*TODO*/,index,))
        })


    unimplemented!("Out of {hands:?}, which hand wins?")
}

fn is_flush(cards: &HashSet<Suit>) -> bool {
    cards.len()==1
}