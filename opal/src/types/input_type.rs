use crate::{SettingSelect, TotalMsg};

use super::setting_option::SettingDuration;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct LabelText(pub (String));

impl Default for LabelText {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Into<LabelText> for &'static str {
    fn into(self) -> LabelText {
        LabelText(self.to_string())
    }
}
impl Into<LabelText> for String {
    fn into(self) -> LabelText {
        LabelText(self)
    }
}
impl From<LabelText> for String {
    fn from(x: LabelText) -> Self {
        x.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputValue(pub TotalMsg);

#[derive(Clone, Debug, PartialEq)]
pub struct InputDuration(pub SettingDuration, pub TotalMsg);

#[derive(Clone, Debug, PartialEq)]
pub struct InputSelect(pub TotalMsg);

#[derive(Clone, Debug, PartialEq)]
pub struct InputMultiSelect(pub Vec<TotalMsg>);

#[derive(Clone, Debug, PartialEq)]
pub struct InputClick(pub TotalMsg);

pub enum InputType {
    ValueSelect(LabelText, InputValue, InputSelect),
    ValueSelectDuration(LabelText, InputValue, InputSelect, InputDuration),
    Select(LabelText, InputSelect),
    Button(LabelText, InputClick),
    Value(LabelText, InputValue),
    MultiSelectSelect(LabelText, InputSelect, InputMultiSelect),
}

impl InputType {
    pub fn warp(
        &self,
    ) -> (
        Option<LabelText>,
        Option<InputValue>,
        Option<InputSelect>,
        Option<InputDuration>,
        Option<InputClick>,
        Option<InputMultiSelect>,
    ) {
        let a = match self {
            InputType::ValueSelect(a, ..)
            | InputType::ValueSelectDuration(a, ..)
            | InputType::Button(a, ..)
            | InputType::Value(a, ..)
            | InputType::Select(a, ..)
            | InputType::MultiSelectSelect(a, _, _) => Some(a.clone()),
        };

        let b = match self {
            InputType::ValueSelect(_, b, ..)
            | InputType::ValueSelectDuration(_, b, ..)
            | InputType::Value(_, b, ..) => Some(b.clone()),
            _ => None,
        };
        let c = match self {
            InputType::ValueSelect(_, _, i, ..)
            | InputType::Select(_, i)
            | InputType::MultiSelectSelect(_, i, _)
            | InputType::ValueSelectDuration(_, _, i, ..) => Some(i.clone()),
            _ => None,
        };
        let d = match self {
            InputType::ValueSelectDuration(_, _, _, i, ..) => Some(i.clone()),
            _ => None,
        };
        let e = match self {
            InputType::Button(_, i) => Some(i.clone()),
            _ => None,
        };
        let f = match self {
            InputType::MultiSelectSelect(_, _, i) => Some(i.clone()),
            _ => None,
        };
        (a, b, c, d, e, f)
    }
}
