use crate::traits::filter_by_coll::FilterByColl;
use crate::FundingColl;
use crate::InputValue;
use crate::LabelText;
use crate::ParserError;
use crate::SetTargetColl;
use crate::SettingList;
use multimap::MultiMap;
use opal_derive::WidgetMsg;
use std::fmt::Display;
use std::str::FromStr;

use crate::{AsInputType, InputType};

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("CollCard")]
#[page("coll_card")]
pub enum LabelMsg {
    UpdateInputLabelValue(Option<String>),
    RemoveInputLabelValue(Option<String>),
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
type Slug = String;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Label {
    pub setting: LabelSetting,
    pub current: MultiMap<LabelText, Slug>,
}
impl Label {
    pub fn get_from_coll_name(&self, slug: &String) -> Vec<LabelText> {
        self.current
            .keys()
            .filter_map(|key| match self.current.get_vec(key) {
                Some(xs) if xs.contains(&slug) => Some(key.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}

impl SettingList for Label {
    type T = String;
    fn push(&self, setting: Self::T) -> Self {
        let mut current_label = self.current.clone();
        current_label.insert(setting.clone().into(), self.setting.slug.clone());

        let mut label = self.clone();
        label.current = current_label;
        label
    }

    fn remove(&self, setting: Self::T) -> Self {
        let mut current_label = self.current.clone();
        current_label.retain(|k, v| (k.0 == setting && v == &self.setting.slug) == false);

        let mut label = self.clone();
        label.current = current_label;
        label
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
        InputType::Value(
            LabelText("Label".to_string()),
            InputValue(
                LabelMsg::UpdateInputLabelValue(Some(self.setting.input.clone())).to_total_msg(),
            ),
        )
    }
}
