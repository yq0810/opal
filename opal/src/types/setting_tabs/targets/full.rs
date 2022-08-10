use crate::InputSelect;
use crate::LabelText;
use opal_derive::WidgetMsg;

use crate::{AsInputType, InputType};

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Target")]
#[page("target_options")]
pub enum FullMsg {
    Select(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Full {
    pub select: bool,
}

impl AsInputType for Full {
    fn input_type(&self) -> InputType {
        InputType::Select(
            LabelText("ALL (except blocked)".to_string()),
            InputSelect(FullMsg::Select(Some(self.select)).to_total_msg()),
        )
    }
}
