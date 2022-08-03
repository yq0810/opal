use crate::{CollResult, FloorPriceResult};

#[derive(Clone, Debug, PartialEq)]
pub struct CollInfo {
    pub slug: String,
    pub slug_result: CollResult,
    pub floor_price_result: Option<FloorPriceResult>,
    pub volume_in_24h: Option<f64>,
}
