use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{components::setting_card::Msg, SettingDuration};

use super::Strategy;

pub enum OneMsg {
    UpdateVolumeRateValue(Option<i64>),
    UpdateVolumeRateDuration(Option<SettingDuration>),
    UpdateTxCountValue(Option<i64>),
    UpdateTxCountDuration(Option<SettingDuration>),
}

impl Strategy for OneMsg {
    fn msgFn() -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg { Msg::OneOptionUpdate(x) };
        Box::new(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct One {
    pub volume_rate_value: i64,
    pub volume_rate_duration: SettingDuration,
    pub tx_count_value: i64,
    pub tx_count_duration: SettingDuration,
}
