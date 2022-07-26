use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{
    components::strategy_options::Msg, get_value, GetValue, SettingCallback, SettingDuration,
};

#[derive(Clone, Debug, PartialEq)]
pub enum OneMsg {
    UpdateVolumeRateValue(Option<i64>),
    UpdateVolumeRateDuration(Option<SettingDuration>),
    UpdateTxCountValue(Option<i64>),
    UpdateTxCountDuration(Option<SettingDuration>),
}

impl OneMsg {
    pub fn callback(&self) -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg { Msg::OneOptionUpdate(x) };
        Box::new(f)
    }
}
impl GetValue for OneMsg {
    fn get_value(&self) -> String {
        match self {
            OneMsg::UpdateVolumeRateValue(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            OneMsg::UpdateVolumeRateDuration(x) => x.map(|x| x.Display()).unwrap_or_default(),
            OneMsg::UpdateTxCountValue(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            OneMsg::UpdateTxCountDuration(x) => x.map(|x| x.Display()).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct One {
    pub volume_rate_value: i64,
    pub volume_rate_duration: SettingDuration,
    pub tx_count_value: i64,
    pub tx_count_duration: SettingDuration,
}
