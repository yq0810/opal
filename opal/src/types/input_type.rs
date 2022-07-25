use yew::Component;

use crate::{strategys, triggers, SettingCallback, SettingDuration};

use super::strategys_algo;

type LabelText = &'static str;
type LabelValue = String;
type SelectValue = bool;

pub enum TotalMsg {
    StrategyMsg(strategys::Msgs),
    TriggerMsg(triggers::Msgs),
}

impl TotalMsg {
    fn to_callback(&self) -> SettingCallback {
        match self {
            TotalMsg::StrategyMsg(x) => x.to_callback(),
            TotalMsg::TriggerMsg(x) => x.to_callback(),
        }
    }
}

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
