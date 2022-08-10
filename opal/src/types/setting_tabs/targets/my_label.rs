use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use crate::traits::filter_by_coll::FilterByColl;
use crate::InputMultiSelect;
use crate::InputSelect;
use crate::LabelText;
use crate::ParserError;
use crate::SettingSelect;
use log::debug;
use opal_derive::WidgetMsg;

use crate::{AsInputType, InputType};

use super::FullMsg;
use super::MyFavoriteMsg;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyLabelSetting {
    pub label: LabelText,
    pub bool: bool,
}

impl Display for MyLabelSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.label.0, self.bool)
    }
}

impl FromStr for MyLabelSetting {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_s = s.split(",").collect::<Vec<&str>>();
        Ok(Self {
            label: LabelText(str_s.get(0).unwrap().to_string()),
            bool: match str_s.get(1).unwrap() {
                &"true" => true,
                &"false" => false,
                _ => return Err(ParserError::BoolError),
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Target")]
#[page("target_options")]
pub enum MyLabelMsg {
    Select(Option<bool>),
    SelectMulti(Option<MyLabelSetting>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyLabel {
    pub select: bool,
    pub selected_labels: HashSet<LabelText>,
    pub total_labels: HashSet<LabelText>,
}

impl FilterByColl for MyLabel {
    fn filter_by_coll(&self, coll: &crate::FundingColl) -> bool {
        coll.labels
            .iter()
            .any(|coll_label| self.selected_labels.contains(coll_label))
    }
}

impl AsInputType for MyLabel {
    fn input_type(&self) -> InputType {
        let select_msg = self
            .total_labels
            .iter()
            .map(|x| MyLabelSetting {
                label: x.clone(),
                bool: self.selected_labels.contains(x),
            })
            .map(|x| MyLabelMsg::SelectMulti(x.to_string().parse().ok()).to_total_msg())
            .collect::<Vec<_>>();

        InputType::MultiSelectSelect(
            LabelText("My Label".to_string()),
            InputSelect(MyLabelMsg::Select(Some(self.select)).to_total_msg()),
            InputMultiSelect(select_msg),
        )
    }
}
