use crate::{
    strategys::Msgs, AsInputType, AsTotalMsg, InputType, SettingDuration, TotalMsg, ValueOP,
};
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};

#[derive(Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro)]
#[totalMsgName("Strategy")]
pub enum ThreeMsg {
    UpdateTxCountValue(Option<i64>),
    UpdateTxCountDuration(Option<SettingDuration>),
    UpdateTxCountSelect(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Three {
    pub tx_count_value: i64,
    pub tx_count_duration: SettingDuration,
    pub tx_count_select: bool,
}

impl AsInputType for Three {
    fn input_type(&self) -> InputType {
        InputType::SelectValueDuration(
            (
                "Tx Count",
                ThreeMsg::UpdateTxCountValue(Some(self.tx_count_value)).to_total_msg(),
            ),
            ThreeMsg::UpdateTxCountSelect(Some(self.tx_count_select)).to_total_msg(),
            (
                self.tx_count_duration.clone(),
                ThreeMsg::UpdateTxCountDuration(Some(self.tx_count_duration.clone()))
                    .to_total_msg(),
            ),
        )
    }
}
