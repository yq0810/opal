use yew::Component;

use crate::{strategys, triggers, SettingCallback, SettingDuration, TotalMsg};

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

pub trait InputTypeExt {
    fn input_type(&self) -> InputType;
}
