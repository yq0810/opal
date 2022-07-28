use crate::strategys::Msgs;
use crate::TotalMsg;
use crate::ValueOP;
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

use crate::{AsInputType, AsTotalMsg, InputType, SettingDuration};

#[derive(Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro)]
#[totalMsgName("Strategy")]
pub enum OneMsg {
    UpdateVolumeRateValue(Option<i64>),
    UpdateVolumeRateDuration(Option<SettingDuration>),
    UpdateVolumeRateSelect(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct One {
    pub volume_rate_value: i64,
    pub volume_rate_duration: SettingDuration,
    pub volume_rate_select: bool,
}

impl AsInputType for One {
    fn input_type(&self) -> InputType {
        InputType::SelectValue(
            (
                "Volume rate",
                OneMsg::UpdateVolumeRateValue(Some(self.volume_rate_value)).to_total_msg(),
            ),
            OneMsg::UpdateVolumeRateSelect(Some(self.volume_rate_select)).to_total_msg(),
        )
    }
}
