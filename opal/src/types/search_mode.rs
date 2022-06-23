pub type SearchResults = Vec<String>;

use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};

use chrono::NaiveDateTime;
use concat_string::concat_string;
use indexmap::IndexSet;
use js_sys::Function;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaQueryList;
use yew::prelude::*;

use opal_derive::Sqlgogo;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use super::QueryError;

#[derive(Clone, Copy)]
pub enum SearchMode {
    Normal,
}
impl SearchMode {
    pub fn placeholder_text(&self) -> &'static str {
        match self {
            SearchMode::Normal => "collection slug",
        }
    }

    pub fn button_text(&self) -> &'static str {
        match self {
            SearchMode::Normal => "Target",
        }
    }
}


pub type QueryOutput = Pin<Box<dyn Future<Output = Result<Vec<String>, QueryError>>>>;
pub trait Query {
    fn sql_query_raw(&self) -> String;
    fn exec_query(r:Self) -> QueryOutput;
}

pub trait SQLResult: where for<'a> Self: Deserialize<'a> {
    fn from_entrys(res: JsValue) -> Vec<String>
    where
        Self: Sized , for<'a> Self: Deserialize<'a> {
        js_sys::Array::from(&res)
            .iter()
            .filter_map(|entry| {
                if let Ok(r) = entry.into_serde::<Self>() {
                    Some(r)
                } else {
                    None
                }
            })
            .map(|x| x.display())
            .collect::<Vec<_>>()
    }
    fn display(&self) -> String;
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FloorPriceResult {
    pub slug: String,
    pub price: f64,
    pub create_time: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TargetResult {
    pub slug: String,
    pub price: f64,
    pub create_time: String,
}

impl SQLResult for TargetResult {
    fn display(&self) -> String {
        // let a =  SearchQuery::FloorPrcieBySlug("hi".to_string());
        format!("Target: {} /  {} ", self.price, self.create_time)
    }
}

impl SQLResult for FloorPriceResult {
    fn display(&self) -> String {
        format!("FloorPrcie: {} /  {}", self.price, self.create_time)
    }
}


#[derive(Clone,Sqlgogo)]
pub enum SearchQuery {
    FloorPriceBySlug(String),
    Target,
}

pub trait Entrys {
    fn entrys(&self,js: JsValue) -> Vec<String>;
}



impl Query for SearchQuery {
    fn sql_query_raw(&self) -> String {
        match self {
            SearchQuery::FloorPriceBySlug(s) => {
                concat_string!("SELECT * FROM floorPrices WHERE slug = \"", s, "\";")
            },
            SearchQuery::Target => {
                concat_string!("SELECT * FROM Targets;")
            },
        }
    }

    fn exec_query(r:Self) -> QueryOutput {
        let query = r.sql_query_raw();
        let fut = async move {
            let res = match exec_query(query).await {
                Ok(res) => Ok(res),
                Err(err) => Err(QueryError::QueryExecError(err)),
            }?;
            // .or(|err|Err(QueryError::QueryExecError(err)));
            let display_entrys = r.entrys(res);
            Ok::<Vec<String>, QueryError>(display_entrys)
        };
        Box::pin(fut)
    }
}
