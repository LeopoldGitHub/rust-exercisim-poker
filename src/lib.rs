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
    let mut hands_vec=Vec::new();
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
                    // puts the suit of the card into a hashSet
                    suit_hash
                        .insert(Suit::new(&card));
                }
                );
            // gets a vec out of the hashmap to enable sorting
            let mut ranks_vec: Vec<(Rank, i32)> = ranks_hash.into_iter().collect();
            // now sorts the vec first by the i32( count of the Rank in a hand)
            // and then by the Rank itself
            ranks_vec.sort_by(|a, b| {
                b.1.cmp(&a.1)
                    .then_with(|| b.0.cmp(&a.0))
            });
            // creates a new Hand and pushes it into the vec for it
            hands_vec.push(
                Hand::new(
                    is_flush(&suit_hash),
                    HandStrength::new(
                        &ranks_vec,
                        is_flush(&suit_hash)),
                    ranks_vec,
                    index))
        });





    unimplemented!("Out of {hands:?}, which hand wins?")
}

fn is_flush(suits: &HashSet<Suit>) -> bool {
    suits.len() == 1
}