use yew::{html::Scope, Component};

use crate::{
    strategys, triggers, AsSettingOption, SettingCallback, SettingDuration, SettingOption, TotalMsg,
};

use super::strategys_algo;

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

impl InputType {}

pub trait InputTypeExt {
    fn input_type(&self) -> InputType;
}
