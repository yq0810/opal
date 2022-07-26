use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::triggers::Msgs;
use crate::{components::trigger_options::Msg, SettingCallback, SettingDuration, TotalMsg};
use crate::{GetValue, InputTypeExt};

#[derive(Clone, Debug, PartialEq)]
pub enum T2Msg {
    UpdatePercentage(Option<u32>),
    UpdateActive(Option<bool>),
}

impl GetValue for T2Msg {
    fn get_value(&self) -> String {
        match self {
            T2Msg::UpdatePercentage(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            T2Msg::UpdateActive(x) => x.map(|x| x.to_string()).unwrap_or_default(),
        }
    }
    fn to_total_msg(&self) -> TotalMsg {
        TotalMsg::TriggerMsg(Msgs::T2(self.clone()))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct T2 {
    pub percentage: u32,
    pub active: bool,
}

impl InputTypeExt for T2 {
    fn input_type(&self) -> crate::InputType {
        crate::InputType::SelectValue(
            (
                "percentage",
                T2Msg::UpdatePercentage(Some(self.percentage)).to_total_msg(),
            ),
            T2Msg::UpdateActive(Some(self.active)).to_total_msg(),
        )
    }
}
