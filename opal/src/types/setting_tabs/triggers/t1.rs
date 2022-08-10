use crate::{AsInputType, InputSelect, InputType, InputValue, LabelText};
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Trigger")]
#[page("trigger_options")]
pub enum T1Msg {
    UpdatePercentage(Option<u32>),
    UpdateActive(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct T1 {
    pub percentage: u32,
    pub active: bool,
}

impl AsInputType for T1 {
    fn input_type(&self) -> InputType {
        InputType::ValueSelect(
            LabelText("T1 FloorPrice %".to_string()),
            InputValue(T1Msg::UpdatePercentage(Some(self.percentage)).to_total_msg()),
            InputSelect(T1Msg::UpdateActive(Some(self.active)).to_total_msg()),
        )
    }
}
