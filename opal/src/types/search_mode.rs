use crate::types::unit::my_date_format;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;
use opal_derive::Sqlgogo;

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

pub trait SQLResult {
    fn from_entrys<T>(res: JsValue) -> Vec<T>
    where
        Self: Sized,
        for<'a> T: Deserialize<'a>,
    {
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
    #[serde(with = "my_date_format")]
    pub create_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TargetResult {
    pub tx_hash: String,
    pub slug: CollResult,
    pub price: f64,
    pub compare_fp: Option<FloorPriceResult>,
    pub compare_ap: ActivePriceResult,
    #[serde(with = "my_date_format")]
    pub create_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivePriceResult {
    pub tx_hash: String,
    pub slug: String,
    pub price: f64,
    #[serde(with = "my_date_format")]
    pub trade_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CollResult {
    pub slug: String,
    pub fee: i32,
}
impl SQLResult for CollResult {
    fn display(&self) -> String {
        format!("Coll: {} ,fee={} ", self.slug, self.fee)
    }
}
impl SQLResult for TargetResult {
    fn display(&self) -> String {
        format!(
            "Target: {:?} , {} , {} , {} = with {} {}",
            self.slug,
            self.price,
            self.create_time,
            self.tx_hash,
            if self.compare_fp.is_some(){
                self.compare_fp.clone().unwrap().display()
            }else{
                "".to_string()
            },
            self.compare_ap.clone().display()
        )
    }
}

impl SQLResult for FloorPriceResult {
    fn display(&self) -> String {
        format!("FloorPrcie: {} , {}", self.price, self.create_time)
    }
}

impl SQLResult for ActivePriceResult {
    fn display(&self) -> String {
        let times = self.trade_time.clone().to_string();
        let times = times.split(".").collect::<Vec<_>>();
        format!("ActivePrice: {} , {:?} , {}", self.price, times.first().unwrap(),self.tx_hash)
    }
}

#[derive(Clone, Sqlgogo)]
pub enum SearchQuery {
    // FloorPriceBySlug(String),
    Target,
    FloorPrice,
    ActivePrice,
    Coll,
    // ActivePriceBySlug(String)
}

pub trait Entrys {
    fn entrys<T>(&self, js: JsValue) -> Vec<T>
    where
        for<'a> T: Deserialize<'a>;
}
