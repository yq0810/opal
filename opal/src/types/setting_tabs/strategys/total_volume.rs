use crate::LabelText;
use crate::{AsInputType, InputSelect, InputType, InputValue};
use opal_derive::WidgetMsg;

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Strategy")]
#[page("strategy_options")]
pub enum TotalVolumeMsg {
    UpdateVolumeTotalValue(Option<f64>),
    UpdateVolumeTotalSelect(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TotalVolume {
    pub volume_total_value: f64,
    pub volume_total_select: bool,
}

impl AsInputType for TotalVolume {
    fn input_type(&self) -> InputType {
        InputType::ValueSelect(
            LabelText("Total volume".to_string()),
            InputValue(
                TotalVolumeMsg::UpdateVolumeTotalValue(Some(self.volume_total_value))
                    .to_total_msg(),
            ),
            InputSelect(
                TotalVolumeMsg::UpdateVolumeTotalSelect(Some(self.volume_total_select))
                    .to_total_msg(),
            ),
        )
    }
}
