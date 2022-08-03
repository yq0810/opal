use crate::area::Msgs;
use crate::components::coll_card;
use crate::ParserError;
use crate::SetTargetColl;
use crate::SettingCallbackFn;
use crate::SettingList;
use crate::TotalMsg;
use crate::ValueOP;
use concat_string::concat_string;
use multimap::MultiMap;
use opal_derive::CallbackMsgMacro;
use opal_derive::SettingCallbackFnMacro;
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::components::coll_card::Msg as PMsg;
use crate::{AsInputType, AsTotalMsg, InputType, SettingDuration};

#[derive(
    Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro, CallbackMsgMacro, SettingCallbackFnMacro,
)]
#[totalMsgName("CollCard")]
#[page("coll_card")]
pub enum LabelMsg {
    Input(Option<LabelSetting>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelSetting {
    pub slug: String,
    pub input: String,
}

impl Display for LabelSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.slug, self.input)
    }
}

impl FromStr for LabelSetting {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_s = s.split(",").collect::<Vec<&str>>();
        Ok(Self {
            slug: str_s.get(0).unwrap().to_string(),
            input: str_s.get(1).unwrap().to_string(),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Label {
    pub setting: LabelSetting,
    pub current: HashMap<String, LabelSetting>,
}
impl SettingList for Label {
    type T = LabelSetting;
    fn push_setting(&self, setting: Self::T) -> Self {
        let mut current_Label = self.current.clone();
        current_Label.insert(setting.slug.clone(), setting.clone());

        let mut Label = self.clone();
        Label.current = current_Label;
        Label
    }

    fn remove_setting(&self, setting: Self::T) -> Self {
        let mut current_Label = self.current.clone();
        current_Label.remove(&setting.slug);

        let mut Label = self.clone();
        Label.current = current_Label;
        Label
    }
}

impl SetTargetColl for LabelSetting {
    fn set_target_coll(&self, target_coll: &String) -> Self {
        let mut setting = self.clone();
        setting.slug = target_coll.clone();
        setting
    }
}

impl AsInputType for Label {
    fn input_type(&self) -> InputType {
        InputType::Value((
            "Label",
            LabelMsg::Input(Some(self.setting.clone())).to_total_msg(),
        ))
    }
}
