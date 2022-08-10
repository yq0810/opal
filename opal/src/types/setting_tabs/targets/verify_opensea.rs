use crate::InputSelect;
use crate::LabelText;
use opal_derive::WidgetMsg;

use crate::{AsInputType, InputType};

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Target")]
#[page("target_options")]
pub enum VerifyOpenseaMsg {
    Select(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VerifyOpensea {
    pub select: bool,
}

impl AsInputType for VerifyOpensea {
    fn input_type(&self) -> InputType {
        InputType::Select(
            LabelText("Verify Opensea".to_string()),
            InputSelect(VerifyOpenseaMsg::Select(Some(self.select)).to_total_msg()),
        )
    }
}
