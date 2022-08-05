use crate::targets::Msgs;
use crate::InputSelect;
use crate::LabelText;
use crate::SettingCallbackFn;
use opal_derive::CallbackMsgMacro;
use opal_derive::SettingCallbackFnMacro;
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

use crate::{components, AsInputType, AsTotalMsg, InputType};

#[derive(
    Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro, CallbackMsgMacro, SettingCallbackFnMacro,
)]
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
            LabelText("ALL".to_string()),
            InputSelect(FullMsg::Select(Some(self.select)).to_total_msg()),
        )
    }
}
