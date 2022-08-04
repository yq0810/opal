use crate::triggers::Msgs;
use crate::TotalMsg;
use crate::ValueOP;
use crate::{AsInputType, AsTotalMsg};
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

#[derive(Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro)]
#[totalMsgName("Trigger")]
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
        crate::InputType::SelectValue(
            (
                "T2 Profit %",
                T2Msg::UpdatePercentage(Some(self.percentage)).to_total_msg(),
            ),
            T2Msg::UpdateActive(Some(self.active)).to_total_msg(),
        )
    }
}
