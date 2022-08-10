use crate::InputSelect;
use crate::InputValue;
use crate::LabelText;

use crate::AsInputType;
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Trigger")]
#[page("trigger_options")]
pub enum T2Msg {
    UpdatePercentage(Option<u32>),
    UpdateActive(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct T2 {
    pub percentage: u32,
    pub active: bool,
}

impl AsInputType for T2 {
    fn input_type(&self) -> crate::InputType {
        crate::InputType::ValueSelect(
            LabelText("T2 Profit %".to_string()),
            InputValue(T2Msg::UpdatePercentage(Some(self.percentage)).to_total_msg()),
            InputSelect(T2Msg::UpdateActive(Some(self.active)).to_total_msg()),
        )
    }
}
