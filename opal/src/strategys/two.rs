use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{components::strategy_options::Msg, SettingCallback, SettingDuration};

#[derive(Clone, Debug, PartialEq)]
pub enum TwoMsg {
    UpdateVolumeTotalValue(Option<f64>),
}

impl SettingCallback<Msg> for TwoMsg {
    fn msgFn() -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg { Msg::TwoOptionUpdate(x) };
        Box::new(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Two {
    pub volume_total_value: f64,
}
