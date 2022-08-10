use crate::{
    AsInputType, InputDuration, InputSelect, InputType, InputValue, LabelText, SettingDuration,
};
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Strategy")]
#[page("strategy_options")]
pub enum ThreeMsg {
    UpdateTxCountValue(Option<i64>),
    UpdateTxCountDuration(Option<SettingDuration>),
    UpdateTxCountSelect(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Three {
    pub tx_count_value: i64,
    pub tx_count_duration: SettingDuration,
    pub tx_count_select: bool,
}

impl AsInputType for Three {
    fn input_type(&self) -> InputType {
        InputType::ValueSelectDuration(
            LabelText("Tx Count".to_string()),
            InputValue(ThreeMsg::UpdateTxCountValue(Some(self.tx_count_value)).to_total_msg()),
            InputSelect(ThreeMsg::UpdateTxCountSelect(Some(self.tx_count_select)).to_total_msg()),
            InputDuration(
                self.tx_count_duration.clone(),
                ThreeMsg::UpdateTxCountDuration(Some(self.tx_count_duration.clone()))
                    .to_total_msg(),
            ),
        )
    }
}
