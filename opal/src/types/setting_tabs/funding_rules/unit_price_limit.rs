use crate::InputSelect;
use crate::InputValue;
use crate::LabelText;

use crate::AsInputType;
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("FundingRule")]
#[page("funding_rule_options")]
pub enum UnitPriceLimitMsg {
    UpdateActive(Option<bool>),
    UpdateValue(Option<u32>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnitPriceLimit {
    pub value: u32,
    pub active: bool,
}

impl AsInputType for UnitPriceLimit {
    fn input_type(&self) -> crate::InputType {
        crate::InputType::ValueSelect(
            LabelText("Unit Price Limit".to_string()),
            InputValue(UnitPriceLimitMsg::UpdateValue(Some(self.value)).to_total_msg()),
            InputSelect(UnitPriceLimitMsg::UpdateActive(Some(self.active)).to_total_msg()),
        )
    }
}
