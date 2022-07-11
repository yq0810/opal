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
    aps.into_iter().filter_map(|target| {
        match (ap_map.get_vec(&target.slug) , coll_map.get(&target.slug)) {
            (Some(ax),Some(coll)) => {
                let ap = ax.iter()
                                                      .cloned()
                                                      .find(|a_after| {
                    let pp = target.price; 
                    let fee = coll.fee as f64 / 100.0;
                    let diff_p = fee + profit_percentage as f64;
                    //todo
                    // let fp = f.price;
                    let fp = a_after.price;
                    let is_base_price_after= &a_after.trade_time > &target.trade_time && a_after.price > target.price;
                    let is_diff_p = is_base_price_after && 
                                           pp < (fp * ((100.0 - diff_p as f64 )/100.0));
                    is_diff_p
                });
                match ap {
                    Some(ap) => {
                        Some(TargetResult {
                            tx_hash: target.tx_hash.clone(),
                            slug: coll.clone(),
                            price: target.price.clone(),
                            create_time: target.trade_time.clone(),
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
        .filter_map(|target| {
            let f = find_first_floor_price(&target.slug, &fps, &target.trade_time);
            match (f,colls.get(&target.slug)) {
                (Some(f),Some(coll)) if f.price > target.price => {
                    let pp = target.price; 
                    let fp = f.price;
                    let is_dff_p = pp < (fp * ((100.0 - diff_percentage as f64 )/100.0));
                    if is_dff_p
                    {
                        Some(TargetResult {
                            tx_hash: target.tx_hash.clone(),
                            slug: coll.clone(),
                            price: target.price.clone(),
                            create_time: target.trade_time.clone(),
                            compare_fp: Some(f),
                            compare_ap: target.clone(),
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
