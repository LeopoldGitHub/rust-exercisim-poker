use crate::hands::card::Rank;

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
    pub(crate) fn new(cards:Vec<(Rank, i32)>, isflush:bool) -> Self {
    //     TODO create me
        HandStrength::RoyalFlush
    }

}