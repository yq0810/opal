use crate::{ActivePriceResult, CollResult, FloorPriceResult};

pub struct MarketData {
    pub fps: Vec<FloorPriceResult>,
    pub aps: Vec<ActivePriceResult>,
    pub colls: Vec<CollResult>,
}
