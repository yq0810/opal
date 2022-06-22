pub type SearchResults = Vec<String>;


use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};

use concat_string::concat_string;
use indexmap::IndexSet;
use js_sys::Function;
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaQueryList;
use yew::prelude::*;
use chrono::NaiveDateTime;
use sql_js_httpvfs_rs::*;
use serde::de::DeserializeOwned;


#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

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


pub trait Query {
     type Output: Future<Output = Vec<String>>;
     fn query(r:SearchQuery) -> Self::Output;
            // R: SQLResult + DeserializeOwned;

}

pub trait SQLResult {
    fn Display(&self) -> String;
}

#[derive(Deserialize,Debug, Clone, PartialEq)]
pub struct FloorResult {
    pub slug: String,
    pub price: f64,
    pub create_time: String
}

impl SQLResult for FloorResult {
    fn Display(&self) -> String {
        format!("FloorPrcie: {} /  {}",self.price,self.create_time)
    }
}


#[derive(Clone)]
pub enum SearchQuery {
    FloorPrcieWithSlug(String,Option<Vec<FloorResult>>)
}

impl Query for SearchQuery 
{
    type Output = Pin<Box<dyn Future<Output = Vec<String>>>>;

    fn query(r:SearchQuery) -> Self::Output
          {
            
            match r {
                SearchQuery::FloorPrcieWithSlug(s, _) => {
                    let query = concat_string!("SELECT * FROM floorPrices WHERE slug = \"", s, "\";");
                    let fut = async move {
                        let mut result = vec![];
                        unsafe {
                            let res_o = exec_query(query).await;
                            match res_o{
                                Ok(res) => {
                                    js_sys::Array::from(&res).iter().for_each(|entry| {
                                        if let Ok(r) = entry.into_serde::<FloorResult>() {
                                            result.push(r.Display())
                                        }else{
                                        }
                                    });
                                },
                                Err(_) => todo!(),
                            }
                        }
                        result
                    };
                    Box::pin(fut)
                },
            }
    }

}
