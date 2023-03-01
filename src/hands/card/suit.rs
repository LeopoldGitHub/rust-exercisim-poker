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