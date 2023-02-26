pub(crate) struct Card {
    rank: Rank,
    suit: Suit,
}

pub(crate) enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

pub(crate) enum Rank {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Rank {
    pub(crate) fn new(card: &str) -> Rank {
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
        }
    }
}

impl Suit {
    pub(crate) fn new(card: &str) -> Self {
        match card.to_uppercase().as_bytes() {
            [.., 67] => Self::Clubs,
            [.., 72] => Self::Hearts,
            [.., 83] => Self::Spades,
            [.., 68] => Self::Diamonds,
        }
    }
}

impl Card {
    pub(crate) fn new(card: &str) -> Self {
        Self {
            rank: Rank::new(card),
            suit: Suit::new(card),
        }
    }
    pub(crate) fn get_suit(&self) -> &Suit {
        &self.suit
    }
    pub(crate) fn get_rank(&self) -> &Rank {
        &self.rank
    }
}