use futures::Future;

use crate::{
    traits::Query, ActivePriceResult, CollResult, FloorPriceResult, MarketData, QueryError,
    SearchQuery,
};

pub struct Market {}

impl Market {
    pub fn new() -> Market {
        Market {}
    }

    pub fn get_marketing_data(&self) -> impl Future<Output = Result<MarketData, QueryError>> {
        async move {
            let colls = SearchQuery::exec_query::<CollResult>(SearchQuery::Coll).await?;
            let fps = SearchQuery::exec_query::<FloorPriceResult>(SearchQuery::FloorPrice).await?;
            let aps =
                SearchQuery::exec_query::<ActivePriceResult>(SearchQuery::ActivePrice).await?;
            Ok(MarketData { fps, aps, colls })
        }
    }
}
