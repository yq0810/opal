use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::triggers::Msgs;
use crate::GetValue;
use crate::{components::trigger_options::Msg, SettingCallback, SettingDuration, TotalMsg};

#[derive(Clone, Debug, PartialEq)]
pub enum T2Msg {
    UpdatePercentage(Option<f64>),
    UpdateActive(Option<bool>),
}

impl GetValue for T2Msg {
    fn get_value(&self) -> String {
        match self {
            T2Msg::UpdatePercentage(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            T2Msg::UpdateActive(x) => x.map(|x| x.to_string()).unwrap_or_default(),
        }
    }
}

impl T2Msg {
    pub fn to_total_msg(&self) -> TotalMsg {
        TotalMsg::TriggerMsg(Msgs::T2(self.clone()))
    }

    pub fn new_value(&self, value: String) -> T2Msg {
        match self {
            T2Msg::UpdatePercentage(_) => T2Msg::UpdatePercentage(value.parse().ok()),
            T2Msg::UpdateActive(_) => T2Msg::UpdateActive(value.parse().ok()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct T2 {
    pub percentage: u32,
    pub active: bool,
}
