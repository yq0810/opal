use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{
    components::strategy_options::Msg, get_value, GetValue, InputType, InputTypeExt,
    SettingCallback, SettingDuration,
};

use super::Msgs;

#[derive(Clone, Debug, PartialEq)]
pub enum OneMsg {
    UpdateVolumeRateValue(Option<i64>),
    UpdateVolumeRateDuration(Option<SettingDuration>),
    UpdateVolumeRateSelect(Option<bool>),
}

impl OneMsg {
    pub fn callback(&self) -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg { Msg::OneOptionUpdate(x) };
        Box::new(f)
    }
}
impl GetValue for OneMsg {
    fn get_value(&self) -> String {
        match self.clone() {
            OneMsg::UpdateVolumeRateValue(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            OneMsg::UpdateVolumeRateDuration(x) => x.map(|x| x.Display()).unwrap_or_default(),
            OneMsg::UpdateVolumeRateSelect(x) => x.map(|x| x.to_string()).unwrap_or_default(),
        }
    }

    fn to_total_msg(&self) -> crate::TotalMsg {
        crate::TotalMsg::StrategyMsg(Msgs::One(self.clone()))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct One {
    pub volume_rate_value: i64,
    pub volume_rate_duration: SettingDuration,
    pub volume_rate_select: bool,
}

impl InputTypeExt for One {
    fn input_type(&self) -> InputType {
        InputType::SelectValue(
            (
                "percentage",
                OneMsg::UpdateVolumeRateValue(Some(self.volume_rate_value)).to_total_msg(),
            ),
            OneMsg::UpdateVolumeRateSelect(Some(self.volume_rate_select)).to_total_msg(),
        )
    }
}
