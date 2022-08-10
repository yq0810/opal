use crate::InputSelect;
use crate::LabelText;
use opal_derive::WidgetMsg;

use crate::{AsInputType, InputType};

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Target")]
#[page("target_options")]
pub enum VerifyTwitterMsg {
    Select(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VerifyTwitter {
    pub select: bool,
}

impl AsInputType for VerifyTwitter {
    fn input_type(&self) -> InputType {
        InputType::Select(
            LabelText("Verify Twitter".to_string()),
            InputSelect(VerifyTwitterMsg::Select(Some(self.select)).to_total_msg()),
        )
    }
}
