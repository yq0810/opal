use crate::{CollResult, FloorPriceResult, Label, LabelText};

#[derive(Clone, Debug, PartialEq)]
pub struct FundingColl {
    pub db: CollResult,
    pub labels: Vec<LabelText>,
    pub fp_last: Option<FloorPriceResult>,
}
