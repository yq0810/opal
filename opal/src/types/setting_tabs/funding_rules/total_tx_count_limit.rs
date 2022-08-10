use crate::InputSelect;
use crate::InputValue;
use crate::LabelText;

use crate::AsInputType;
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("FundingRule")]
#[page("funding_rule_options")]
pub enum TotalTxCountLimitMsg {
    UpdateValue(Option<u32>),
    UpdateActive(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TotalTxCountLimit {
    pub value: u32,
    pub active: bool,
}

impl AsInputType for TotalTxCountLimit {
    fn input_type(&self) -> crate::InputType {
        crate::InputType::ValueSelect(
            LabelText("Total Tx Count Limit".to_string()),
            InputValue(TotalTxCountLimitMsg::UpdateValue(Some(self.value)).to_total_msg()),
            InputSelect(TotalTxCountLimitMsg::UpdateActive(Some(self.active)).to_total_msg()),
        )
    }
}
