use crate::components;
use crate::triggers::Msgs;
use crate::InputSelect;
use crate::InputValue;
use crate::LabelText;
use crate::SettingCallbackFn;
use crate::{AsInputType, AsTotalMsg};
use opal_derive::CallbackMsgMacro;
use opal_derive::SettingCallbackFnMacro;
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

#[derive(
    Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro, CallbackMsgMacro, SettingCallbackFnMacro,
)]
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
