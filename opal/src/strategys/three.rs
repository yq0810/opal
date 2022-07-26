use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{
    components::strategy_options::Msg, get_value, GetValue, InputType, InputTypeExt,
    SettingCallback, SettingDuration,
};

use super::Msgs;

#[derive(Clone, Debug, PartialEq)]
pub enum ThreeMsg {
    UpdateTxCountValue(Option<i64>),
    UpdateTxCountDuration(Option<SettingDuration>),
    UpdateTxCountSelect(Option<bool>),
}

impl ThreeMsg {
    pub fn callback(&self) -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg { Msg::ThreeOptionUpdate(x) };
        Box::new(f)
    }
}
impl GetValue for ThreeMsg {
    fn get_value(&self) -> String {
        match self.clone() {
            ThreeMsg::UpdateTxCountValue(x) => x.map(|x| x.to_string()).unwrap_or_default(),
            ThreeMsg::UpdateTxCountDuration(x) => x.map(|x| x.Display()).unwrap_or_default(),
            ThreeMsg::UpdateTxCountSelect(x) => x.map(|x| x.to_string()).unwrap_or_default(),
        }
    }

    fn to_total_msg(&self) -> crate::TotalMsg {
        crate::TotalMsg::StrategyMsg(Msgs::Three(self.clone()))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Three {
    pub tx_count_value: i64,
    pub tx_count_duration: SettingDuration,
    pub tx_count_select: bool,
}

impl InputTypeExt for Three {
    fn input_type(&self) -> InputType {
        InputType::SelectValueDuration(
            (
                "percentage",
                ThreeMsg::UpdateTxCountValue(Some(self.tx_count_value)).to_total_msg(),
            ),
            ThreeMsg::UpdateTxCountSelect(Some(self.tx_count_select)).to_total_msg(),
            (
                self.tx_count_duration.clone(),
                ThreeMsg::UpdateTxCountDuration(Some(self.tx_count_duration.clone()))
                    .to_total_msg(),
            ),
        )
    }
}
