use crate::{bid::BidSide, offer::OfferSide};
#[derive(Debug, Clone)]
pub struct State {
    pub offer_side: OfferSide,
    pub bid_side: BidSide,
}

impl State {
    pub fn new() -> Self {
        return State {
            offer_side: OfferSide::new(),
            bid_side: BidSide::new(),
        };
    }
}
