use yew::{html::Scope, Component};

use crate::{strategys, triggers, AsSettingOption, TotalMsg};

use super::{setting_option::SettingDuration, strategys_algo};

type LabelText = &'static str;
type LabelValue = String;
type SelectValue = bool;

type InputValue = (LabelText, TotalMsg);
type InputDuration = (SettingDuration, TotalMsg);
type InputSelect = (TotalMsg);

pub enum InputType {
    SelectValue(InputValue, InputSelect),
    SelectValueDuration(InputValue, InputSelect, InputDuration),
}

impl InputType {
    pub fn warp(
        &self,
    ) -> (
        Option<InputValue>,
        Option<InputSelect>,
        Option<InputDuration>,
    ) {
        match self {
            InputType::SelectValue(a, b) => (Some(a.clone()), Some(b.clone()), None),
            InputType::SelectValueDuration(a, b, c) => {
                (Some(a.clone()), Some(b.clone()), Some(c.clone()))
            }
        }
    }
}
