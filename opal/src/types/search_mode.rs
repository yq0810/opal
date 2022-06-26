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


pub type QueryOutput<T> = Pin<Box<dyn Future<Output = Result<Vec<T>, QueryError>>>>;
pub trait Query {
    fn sql_query_raw(&self) -> String;
    fn exec_query<T>(r:Self) -> QueryOutput<T> 
      where
        Self: Sized , for<'a> T: Deserialize<'a>;
}

pub trait SQLResult {
    fn from_entrys<T>(res: JsValue) -> Vec<T>
    where
        Self: Sized , for<'a> T: Deserialize<'a> {
        js_sys::Array::from(&res)
            .iter()
            .filter_map(|entry| {
                if let Ok(r) = entry.into_serde::<T>() {
                    Some(r)
                } else {
                    None
                }
            })
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
    pub tx_hash: String,
    pub slug: String,
    pub price: f64,
    pub create_time: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivePriceResult {
    pub tx_hash: String,
    pub slug: String,
    pub price: f64,
    pub trade_time: String,
}

impl SQLResult for TargetResult {
    fn display(&self) -> String {
        // let a =  SearchQuery::FloorPrcieBySlug("hi".to_string());
        format!("Target: {} , {} , {} , {}", self.slug , self.price, self.create_time, self.tx_hash)
    }
}

impl SQLResult for FloorPriceResult {
    fn display(&self) -> String {
        format!("FloorPrcie: {} , {} ",self.price, self.create_time)
    }
}

impl SQLResult for ActivePriceResult {
    fn display(&self) -> String {
        format!("Active: {}",self.price)
    }
}

#[derive(Clone,Sqlgogo)]
pub enum SearchQuery {
    FloorPriceBySlug(String),
    Target,
    ActivePriceBySlug(String)
}

pub trait Entrys {
    fn entrys<T>(&self,js: JsValue) -> Vec<T> 
    where
        for<'a> T: Deserialize<'a>;
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
            SearchQuery::ActivePriceBySlug(s) => {
                concat_string!("SELECT * FROM activePrices WHERE slug = \"", s, "\";")
            },
        }
    }

    fn exec_query<T>(r:Self) -> QueryOutput<T> 
    where 
        for<'a> T: Deserialize<'a> {
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
