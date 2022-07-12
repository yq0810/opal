use crate::{app::Msg, types::unit::my_date_format, Query};
use chrono::{DateTime, Utc};
use futures::Future;
use opal_derive::Sqlgogo;
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

#[derive(Clone, Copy)]
pub enum SearchMode {
    T1,
    T2,
}
impl SearchMode {
    pub fn placeholder_text(&self) -> &'static str {
        match self {
            SearchMode::T1 => " > FloorPrice _ %",
            SearchMode::T2 => " > Profit _ %",
        }
    }

    pub fn button_text(&self) -> &'static str {
        match self {
            SearchMode::T1 => "T1",
            SearchMode::T2 => "T2",
        }
    }

    pub fn start(diff_p: i32) -> impl Future<Output = Msg> {
        async move {
            let msg = SearchQuery::exec_query::<FloorPriceResult>(SearchQuery::FloorPrice).await;
            let msgs2 =
                SearchQuery::exec_query::<ActivePriceResult>(SearchQuery::ActivePrice).await;
            let msgs3 = SearchQuery::exec_query::<CollResult>(SearchQuery::Coll).await;
            Msg::ShowRefresh(
                msg.clone().unwrap(),
                msgs2
                    .clone()
                    .unwrap()
                    .into_iter()
                    .filter(|x| x.price < 500.0)
                    .collect(),
                msgs3.clone().unwrap(),
                diff_p,
            )
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

impl TargetResult {
    pub fn profit_sale_at(&self, ap: &ActivePriceResult) -> Option<f64> {
        // todo
        // let gas
        // Ex: 12.5 = 12.5%
        let sale_fee_percentage = self.slug.fee as f64 / 100.0 / 100.0;
        let net_profit = (ap.price -  (ap.price * sale_fee_percentage)) - self.price;
        return Some(net_profit);
    }
    pub fn profit_p_sale_at(&self, ap: &ActivePriceResult) -> Option<f64> {
        // todo
        // let gas
        // Ex: 12.5 = 12.5%
        let profit = self.profit_sale_at(ap);
        let profit = (self.price + profit.unwrap_or_default()) / self.price * 100.0 - 100.0;
        
        return Some(profit.floor());
    }
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
        format!("{}, sale fee={} ", self.slug, self.fee)
    }
}
impl SQLResult for TargetResult {
    fn display(&self) -> String {
        format!(
            "Target: {} , {} ETH, {} , {}",
            self.slug.display(),
            self.price,
            self.create_time,
            self.tx_hash,
            // if self.compare_fp.is_some(){
            //     self.compare_fp.clone().unwrap().display()
            // }else{
            //     "".to_string()
            // },
            // self.compare_ap.clone().display()
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
        format!(
            "ActivePrice: {} , {:?} , {}",
            self.price,
            times.first().unwrap(),
            self.tx_hash
        )
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
