use crate::{AsInputType, InputSelect, InputType, InputValue, LabelText};
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("FundingRule")]
#[page("funding_rule_options")]
pub enum TotalAmountLimitMsg {
    UpdateValue(Option<u32>),
    UpdateActive(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TotalAmountLimit {
    pub value: u32,
    pub active: bool,
}

impl AsInputType for TotalAmountLimit {
    fn input_type(&self) -> InputType {
        InputType::ValueSelect(
            LabelText("Total Amount Limit".to_string()),
            InputValue(TotalAmountLimitMsg::UpdateValue(Some(self.value)).to_total_msg()),
            InputSelect(TotalAmountLimitMsg::UpdateActive(Some(self.active)).to_total_msg()),
        )
    }
}
