use chrono::{DateTime, Duration, Utc};
use multimap::MultiMap;

use crate::ActivePriceResult;

pub fn slug_active_in_duration(
    date_time: &DateTime<Utc>,
    duration: &Duration,
    slug: &String,
    ap_map: &MultiMap<String, ActivePriceResult>,
) -> Option<Vec<ActivePriceResult>> {
    match ap_map.get_vec(slug) {
        Some(s) => {
            let volume = s
                .iter()
                .filter(|x| {
                    &x.trade_time < date_time && &x.trade_time < &(date_time.clone() + *duration)
                })
                .cloned()
                .collect::<Vec<_>>();
            Some(volume)
        }
        None => None,
    }
}