use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash)]
pub(crate) enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

impl Suit {
    pub(crate) fn new(card: &str) -> Self {
        // matches the ascii of the index to get the Suit but only looks at the last char
        match card.to_uppercase().as_bytes() {
            [.., 67] => Self::Clubs,
            [.., 72] => Self::Hearts,
            [.., 83] => Self::Spades,
            [.., 68] => Self::Diamonds,
            _ => panic!(),
        }
    }
}
// enum of Card Ranks with a given value of 11 Bitwise shifted to the left x times
// to make the check for straights easier with Aces as 1 in a 5 4 3 2 Ace straight
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
pub(crate) enum Rank {
    Ace = (3<<12)+1,
    King = 3<<11,
    Queen = 3<<10,
    Jack = 3<<9,
    Ten = 3<<8,
    Nine = 3<<7,
    Eight = 3<<6,
    Seven = 3<<5,
    Six = 3<<4,
    Five = 3<<3,
    Four = 3<<2,
    Three = 3<<1,
    Two = 3,
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_usize = *self as usize;
        let other_usize = *other as usize;
        self_usize.cmp(&other_usize)
    }
}

impl Rank {
    pub(crate) fn new(card: &str) -> Rank {
        // matches the ascii of the index to get the Rank, does not look at the last char
        match card.to_uppercase().as_bytes() {
            [65, _] => Self::Ace,
            [75, _] => Self::King,
            [81, _] => Self::Queen,
            [74, _] => Self::Jack,
            [49, 48, _] => Self::Ten,
            [57, _] => Self::Nine,
            [56, _] => Self::Eight,
            [55, _] => Self::Seven,
            [54, _] => Self::Six,
            [53, _] => Self::Five,
            [52, _] => Self::Four,
            [51, _] => Self::Three,
            [50, _] => Self::Two,
            _ => panic!(),
        }
    }
}

