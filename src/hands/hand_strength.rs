use crate::hands::card::Rank;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum HandStrength {
    RoyalFlush = 9,
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    Pair = 1,
    HighCard = 0,
}

impl HandStrength {
    // the creator for HandStrength Enum
    pub(crate) fn new(cards: &Vec<(Rank, i32)>, is_flush: bool) -> Self {
        // matches the i32 in of the first 2 indexes in the sorted vec of cards
        match (&cards[0].1, &cards[1].1) {
            // case 4 do not care about second
            (4, _) => Self::FourOfAKind,
            // case 3 and 2 and so on
            (3, 2) => Self::FullHouse,
            (3, _) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPairs,
            (2, _) => Self::Pair,
            // case 1,1 AND is_flush is true
            (1, 1) if is_flush => Self::get_flush_type(cards),
            (1, 1) if Self::is_straight(&cards) => Self::Straight,
            // default case
            _ => Self::HighCard
        }
    }
    fn is_straight(cards: &Vec<(Rank, i32)>) -> bool {
        // iterates over the array in steps of 2 and returns true if they have adjacent values
        cards.windows(2).filter(|a| {
            // (a[0].0 as usize) == (a[1].0 as usize) + 1 || if a[1].0 == Rank::Ace {
            //     (a[0].0 as usize)+12 ==(a[1].0 as usize)
            // } else { false }
            (a[0].0 as usize)&(a[1].0 as usize)>0

        }).count() == 4
    }
    // helper function to get the type of flush
    fn get_flush_type(cards: &Vec<(Rank, i32)>) -> Self {
        // gets the Rank values out of the vec and converts them into their usize value
        let ranks_vec: Vec<usize> = cards.iter().map(|a| a.0 as usize).collect();
        // now matches them
        match ranks_vec[..] {
            [14, 13, 12, 11, 10] => Self::RoyalFlush,
            [..] if Self::is_straight(&cards) => Self::StraightFlush,
            // default case
            _ => Self::Flush,
        }
    }
}