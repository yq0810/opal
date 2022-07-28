use serde::Deserialize;

use crate::QueryOutput;

pub trait Query {
    fn sql_query_raw(&self) -> String;
    fn exec_query<T>(r: Self) -> QueryOutput<T>
    where
        Self: Sized,
        for<'a> T: Deserialize<'a>;
}
