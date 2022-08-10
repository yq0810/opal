use crate::{CollResult, Label, LabelText};

#[derive(Clone, Debug, PartialEq)]
pub struct FundingColl {
    pub db: CollResult,
    pub labels: Vec<LabelText>,
}
