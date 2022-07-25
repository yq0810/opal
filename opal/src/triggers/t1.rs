use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::input_type::InputTypeExt;
use crate::{components::trigger_options::Msg, SettingCallback, SettingDuration};
use crate::{InputType, TotalMsg};

use super::Msgs;

#[derive(Clone, Debug, PartialEq)]
pub enum T1Msg {
    UpdatePercentage(Option<u32>),
    UpdateActive(bool),
}
impl T1Msg {
    pub fn to_total_msg(&self) -> TotalMsg {
        TotalMsg::TriggerMsg(Msgs::T1(self.clone()))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct T1 {
    pub percentage: u32,
    pub active: bool,
}

impl InputTypeExt for T1 {
    fn input_type(&self) -> InputType {
        InputType::SelectValue(
            (
                "percentage",
                T1Msg::UpdatePercentage(Some(self.percentage)).to_total_msg(),
            ),
            T1Msg::UpdateActive(self.active).to_total_msg(),
        )
    }
}
