use crate::traits::AsInputType;
use crate::InputSelect;
use crate::InputType;
use crate::SettingDuration;
use crate::{InputValue, LabelText};
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Strategy")]
#[page("strategy_options")]
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
        InputType::ValueSelect(
            LabelText("Volume rate".to_string()),
            InputValue(OneMsg::UpdateVolumeRateValue(Some(self.volume_rate_value)).to_total_msg()),
            InputSelect(
                OneMsg::UpdateVolumeRateSelect(Some(self.volume_rate_select)).to_total_msg(),
            ),
        )
    }
}
