use chrono::{DateTime, Utc};
use multimap::MultiMap;

use crate::{app::App, ActivePriceResult, FloorPriceResult, TargetResult, CollResult};

pub fn find_first_floor_price(
    slug: &String,
    fps: &MultiMap<String, FloorPriceResult>,
    date_time: &DateTime<Utc>,
) -> Option<FloorPriceResult> {
    match fps.get_vec(slug) {
        Some(xs) => xs
            .iter()
            .find(|x| {
                &x.create_time > date_time
                    && (x.create_time.timestamp() - date_time.timestamp()) < 600
            })
            .and_then(|x| Some(x.clone())),
        None => None,
    }
}

pub fn find_traget_from_profit(
    aps: &Vec<ActivePriceResult>,
    ap_map: &MultiMap<String, ActivePriceResult>,
    coll_map: &MultiMap<String, CollResult>,
    profit_percentage: i32,
) -> Vec<TargetResult> {
    aps.into_iter().filter_map(|x| {
        match (ap_map.get_vec(&x.slug) , coll_map.get(&x.slug)) {
            (Some(ax),Some(coll)) => {
                let ap = ax.iter().cloned().find(|a_after| {
                    &a_after.trade_time > &x.trade_time && a_after.price > x.price 
                        && (a_after.price as f64 - x.price as f64 / x.price as f64
                            > ((profit_percentage as f64)  + (coll.fee as f64 / 100.0 )/ 100.0))

                });
                match ap {
                    Some(ap) => {
                        Some(TargetResult {
                            tx_hash: x.tx_hash.clone(),
                            slug: coll.clone(),
                            price: x.price.clone(),
                            create_time: x.trade_time.clone(),
                            compare_fp: None,
                            compare_ap: ap.clone(),
                        })

                    },
                    None => None,
                }

            },
            _ => None,
        }

    }).collect()
}

pub fn find_traget_from_floor_active(
    fps: &MultiMap<String, FloorPriceResult>,
    colls: &MultiMap<String, CollResult>,
    aps: &Vec<ActivePriceResult>,
    diff_percentage: i32,
) -> Vec<TargetResult> {
    aps.into_iter()
        .filter_map(|x| {
            let f = find_first_floor_price(&x.slug, &fps, &x.trade_time);
            match (f,colls.get(&x.slug)) {
                (Some(f),Some(coll)) => {
                    if x.price < f.price
                        && (x.price as f64 - f.price as f64 / f.price as f64
                            >  ((diff_percentage as f64)  + (coll.fee as f64 / 100.0 )/ 100.0))
                    {
                        Some(TargetResult {
                            tx_hash: x.tx_hash.clone(),
                            slug: coll.clone(),
                            price: x.price.clone(),
                            create_time: x.trade_time.clone(),
                            compare_fp: Some(f),
                            compare_ap: x.clone(),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect()
}