use crate::{
    find_first_floor_price, strategys::slug_active_in_duration, ActivePriceResult, FloorPriceResult,
};
use chrono::{DateTime, Duration, Utc};
use multimap::MultiMap;

#[derive(Copy, Clone, Debug, Default)]
pub struct StrategyOne {
    pub total_volume: f64,
    pub tx_count: i64,
}

pub fn strategy_one(
    date_time: &DateTime<Utc>,
    slug: &String,
    volume_duration: &Duration,
    tx_duration: &Duration,
    ap_map: &MultiMap<String, ActivePriceResult>,
) -> StrategyOne {
    let duration_actives = slug_active_in_duration(date_time, volume_duration, slug, ap_map);
    let total_volume: Option<f64> = duration_actives
        .as_ref()
        .and_then(|x| Some(x.iter().map(|x| x.price).sum()));
    let duration_tx = slug_active_in_duration(date_time, tx_duration, slug, ap_map);
    StrategyOne {
        tx_count: duration_tx
            .as_ref()
            .and_then(|x| Some(x.len() as i64))
            .unwrap_or_default(),
        total_volume: total_volume.unwrap_or_default(),
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct StrategyTwo {
    pub total_volume: f64,
}
pub fn strategy_two(
    date_time: &DateTime<Utc>,
    slug: &String,
    fp_map: &MultiMap<String, FloorPriceResult>,
) -> StrategyTwo {
    let duration_tx = find_first_floor_price(slug, fp_map, date_time);
    StrategyTwo {
        total_volume: duration_tx.map(|x| x.total_volume).unwrap_or_default(),
    }
}
