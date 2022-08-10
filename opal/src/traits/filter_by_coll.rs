use crate::{CollResult, FundingColl};

pub trait FilterByColl {
    fn filter_by_coll(&self, coll: &FundingColl) -> bool;
}
