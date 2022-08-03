use yew::{html::Scope, Component};

use crate::{strategys, triggers, AsSettingOption, TotalMsg};

use super::{setting_option::SettingDuration, strategys_algo};

type LabelText = &'static str;
type LabelText2 = String;
type LabelValue = String;
type SelectValue = bool;

type InputValue = (LabelText, TotalMsg);
type InputDuration = (SettingDuration, TotalMsg);
type InputSelect = TotalMsg;
type InputClick = (LabelText2, TotalMsg);

pub enum InputType {
    Value(InputValue),
    SelectValue(InputValue, InputSelect),
    SelectValueDuration(InputValue, InputSelect, InputDuration),
    Button(InputClick),
}

impl InputType {
    pub fn warp(
        &self,
    ) -> (
        Option<InputValue>,
        Option<InputSelect>,
        Option<InputDuration>,
        Option<InputClick>,
    ) {
        match self {
            InputType::SelectValue(a, b) => (Some(a.clone()), Some(b.clone()), None, None),
            InputType::SelectValueDuration(a, b, c) => {
                (Some(a.clone()), Some(b.clone()), Some(c.clone()), None)
            }
            InputType::Button(a) => (None, None, None, Some(a.clone())),
            InputType::Value(a) => (Some(a.clone()), None, None, None),
        }
    }
}
