use super::Msgs;
use crate::ValueOP;
use crate::{AsInputType, AsTotalMsg, InputType, TotalMsg};
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

#[derive(Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro)]
#[totalMsgName("Strategy")]
pub enum TwoMsg {
    UpdateVolumeTotalValue(Option<f64>),
    UpdateVolumeTotalSelect(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Two {
    pub volume_total_value: f64,
    pub volume_total_select: bool,
}

impl AsInputType for Two {
    fn input_type(&self) -> InputType {
        InputType::SelectValue(
            (
                "Total volume",
                TwoMsg::UpdateVolumeTotalValue(Some(self.volume_total_value)).to_total_msg(),
            ),
            TwoMsg::UpdateVolumeTotalSelect(Some(self.volume_total_select)).to_total_msg(),
        )
    }
}
