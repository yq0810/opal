pub type SearchResults = Vec<Html>;
use std::future::Future;
use std::pin::Pin;
use crate::{SearchQuery, QueryError, Entrys};
use concat_string::concat_string;
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use yew::Html;

pub type QueryOutput<T> = Pin<Box<dyn Future<Output = Result<Vec<T>, QueryError>>>>;
pub trait Query {
    fn sql_query_raw(&self) -> String;
    fn exec_query<T>(r: Self) -> QueryOutput<T>
    where
        Self: Sized,
        for<'a> T: Deserialize<'a>;
}

impl Query for SearchQuery {
    fn sql_query_raw(&self) -> String {
        match self {
            // SearchQuery::FloorPriceBySlug(s) => {
            //     concat_string!("SELECT * FROM floorPrices WHERE slug = \"", s, "\";")
            // },
            SearchQuery::Target => {
                concat_string!("SELECT * FROM Targets;")
            }
            // SearchQuery::ActivePriceBySlug(s) => {
            //     concat_string!("SELECT * FROM activePrices WHERE slug = \"", s, "\";")
            // },
            SearchQuery::FloorPrice => concat_string!("SELECT * FROM floorPrices;"),
            SearchQuery::ActivePrice => concat_string!("SELECT * FROM activePrices;"),
            SearchQuery::Coll => concat_string!("SELECT * FROM colls;"),
        }
    }

    fn exec_query<T>(r: Self) -> QueryOutput<T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let query = r.sql_query_raw();
        let fut = async move {
            let res = match exec_query(query).await {
                Ok(res) => Ok(res),
                Err(err) => Err(QueryError::QueryExecError(err)),
            }?;
            let display_entrys = r.entrys::<T>(res);
            Ok::<Vec<T>, QueryError>(display_entrys)
        };
        Box::pin(fut)
    }
}