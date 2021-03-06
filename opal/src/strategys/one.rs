use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{components::strategy_options::Msg, SettingCallback, SettingDuration};

#[derive(Clone, Debug, PartialEq)]
pub enum OneMsg {
    UpdateVolumeRateValue(Option<i64>),
    UpdateVolumeRateDuration(Option<SettingDuration>),
    UpdateTxCountValue(Option<i64>),
    UpdateTxCountDuration(Option<SettingDuration>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct One {
    pub volume_rate_value: i64,
    pub volume_rate_duration: SettingDuration,
    pub tx_count_value: i64,
    pub tx_count_duration: SettingDuration,
}
