use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{
    components::strategy_options::Msg, GetValue, InputType, InputTypeExt, SettingCallback,
    SettingDuration,
};

use super::Msgs;

#[derive(Clone, Debug, PartialEq)]
pub enum TwoMsg {
    UpdateVolumeTotalValue(Option<f64>),
    UpdateVolumeTotalSelect(Option<bool>),
}

impl GetValue for TwoMsg {
    fn get_value(&self) -> String {
        match self {
            TwoMsg::UpdateVolumeTotalValue(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            TwoMsg::UpdateVolumeTotalSelect(x) => x.map(|x| x.to_string()).unwrap_or_default(),
        }
    }

    fn to_total_msg(&self) -> crate::TotalMsg {
        crate::TotalMsg::StrategyMsg(Msgs::Two(self.clone()))
    }
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
    pub volume_total_select: bool,
}

impl InputTypeExt for Two {
    fn input_type(&self) -> InputType {
        InputType::SelectValue(
            (
                "Total Volume",
                TwoMsg::UpdateVolumeTotalValue(Some(self.volume_total_value)).to_total_msg(),
            ),
            TwoMsg::UpdateVolumeTotalSelect(Some(self.volume_total_select)).to_total_msg(),
        )
    }
}
