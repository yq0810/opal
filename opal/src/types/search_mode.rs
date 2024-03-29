use std::collections::HashMap;

use crate::{
    pages::{Config, Msg},
    strategy_one,
    types::unit::my_date_format,
    CollInfo, Market, PickColl, Query,
};
use chrono::{DateTime, Duration, Utc};
use futures::Future;
use multimap::MultiMap;
use opal_derive::Sqlgogo;
use serde::Deserialize;
use wasm_bindgen::JsValue;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;
use wasm_timer::Delay;

#[derive(Clone, Copy)]
pub enum SearchMode {
    Slug,
}

impl Default for SearchMode {
    fn default() -> Self {
        SearchMode::Slug
    }
}

impl SearchMode {
    pub fn placeholder_text(&self) -> &'static str {
        match self {
            SearchMode::Slug => "slug name",
        }
    }

    pub fn button_text(&self) -> &'static str {
        match self {
            SearchMode::Slug => "Collection",
        }
    }

    pub fn start_slug(slug: String, delay: Duration) -> impl Future<Output = Msg> {
        async move {
            Delay::new(delay.to_std().unwrap()).await.unwrap();
            let msgs3 = SearchQuery::exec_query::<CollResult>(SearchQuery::Coll).await;
            let result = match msgs3 {
                Ok(x) => x.iter().find_map(|x| {
                    if x.slug == slug {
                        Some(x.clone())
                    } else {
                        None
                    }
                }),
                Err(_) => None,
            };

            let msg = SearchQuery::exec_query::<FloorPriceResult>(SearchQuery::FloorPrice).await;
            let result = match (msg, result) {
                (Ok(fps), Some(slug_result)) => {
                    let msg_aps =
                        SearchQuery::exec_query::<ActivePriceResult>(SearchQuery::ActivePrice)
                            .await
                            .unwrap();
                    let last_fps = fps.iter().filter(|x| x.slug == slug).last().cloned();
                    let mut aps: MultiMap<String, ActivePriceResult> = MultiMap::new();
                    msg_aps.iter().for_each(|x| {
                        aps.insert(slug.clone(), x.clone());
                    });

                    // let fp = find_first_floor_price(&slug, &new_map, &);
                    let one = last_fps.clone().map(|x| {
                        strategy_one(
                            &x.create_time,
                            &slug,
                            &Duration::hours(24),
                            &Duration::hours(24),
                            &aps,
                        )
                    });

                    let result = CollInfo {
                        slug,
                        slug_result,
                        floor_price_result: last_fps,
                        volume_in_24h: one.map(|x| x.total_volume),
                    };
                    Some(result)
                }
                _ => None,
            };

            Msg::ShowCollRefresh(result)
        }
    }

    pub fn start(config: Config) -> impl Future<Output = Msg> {
        async move {
            let market = Market::new();
            let market_data = market.get_marketing_data().await.unwrap();
            let mut fps_map: HashMap<String, FloorPriceResult> = HashMap::new();
            market_data.fps.iter().rev().for_each(|x| {
                if !fps_map.contains_key(&x.slug) {
                    fps_map.insert(x.slug.clone(), x.clone());
                }
            });

            let coll = PickColl {
                config: crate::PickCollConfig {
                    target_config: config.setting_card.target.clone(),
                    area: config.area.clone(),
                    strategy: config.setting_card.strategy.clone(),
                },
                colls: market_data.colls,
                fps_map,
            };
            let (c, c2) = coll.run();
            Msg::PickedAndExceptedColls(c, c2)
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
    pub total_volume: f64,
    #[serde(with = "my_date_format")]
    pub create_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TargetResult {
    pub tx_hash: String,
    pub slug: CollResult,
    pub price: f64,
    pub gas_price: f64,
    pub gas_used: i64,
    pub compare_fp: Option<FloorPriceResult>,
    pub compare_ap: ActivePriceResult,
    #[serde(with = "my_date_format")]
    pub create_time: DateTime<Utc>,
}

impl TargetResult {
    pub fn total_cost(&self, ap: &ActivePriceResult) -> f64 {
        let buy_price = self.price;
        let block_cost = self.gas_price * self.gas_used as f64 * 0.000000001;
        let sale_fee_percentage = ap.price * self.slug.fee as f64 / 100.0 / 100.0;
        buy_price + block_cost + sale_fee_percentage
    }

    pub fn profit_sale_at(&self, ap: &ActivePriceResult) -> Option<f64> {
        let net_profit = ap.price - self.total_cost(ap);
        return Some(net_profit);
    }

    pub fn profit_p_sale_at(&self, ap: &ActivePriceResult) -> Option<f64> {
        let profit = self.profit_sale_at(ap);
        let profit =
            (self.price + profit.unwrap_or_default()) / self.total_cost(ap) * 100.0 - 100.0;

        return Some(profit.floor());
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ActivePriceResult {
    pub tx_hash: String,
    pub slug: String,
    pub price: f64,
    pub gas_price: f64,
    pub gas_used: i64,
    #[serde(with = "my_date_format")]
    pub trade_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CollResult {
    pub slug: String,
    pub fee: i32,
    pub is_verified: i32,
    pub twitter_is_verified: i32,
}
impl SQLResult for CollResult {
    fn display(&self) -> String {
        format!("{}, sale fee={} ", self.slug, self.fee)
    }
}
impl SQLResult for TargetResult {
    fn display(&self) -> String {
        format!(
            "Target: {} , {} ETH, {} , {}, Gas Cost: {} Gwei * {} = {} ETH",
            self.slug.display(),
            self.price,
            self.create_time,
            self.tx_hash,
            self.gas_price,
            self.gas_used,
            self.gas_price * self.gas_used as f64 * 0.000000001,
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
