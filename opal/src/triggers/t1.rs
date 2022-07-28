use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

use super::Msgs;
use crate::{as_total_msg, AsInputType, AsTotalMsg, InputType, TotalMsg, ValueOP};

#[derive(Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro)]
#[totalMsgName("Trigger")]
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
        InputType::SelectValue(
            (
                "T1 FloorPrice %",
                T1Msg::UpdatePercentage(Some(self.percentage)).to_total_msg(),
            ),
            T1Msg::UpdateActive(Some(self.active)).to_total_msg(),
        )
    }
}
