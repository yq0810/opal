use yew::html::ImplicitClone;

use crate::{
    components::{
        strategy_options::{self, StrategyOptions},
        trigger_options::{self, TriggerOptions},
    },
    strategys, triggers, SettingDurationToggle, SettingOption, SettingValueInput,
};

#[derive(Clone, PartialEq)]
pub enum InputsProps {
    Trigger(SettingOption<trigger_options::Msg, triggers::Msgs, TriggerOptions>),
    Strategy(SettingOption<strategy_options::Msg, strategys::Msgs, StrategyOptions>),
}

impl InputsProps {
    pub fn option_input_data(&self) -> String {
        match self {
            InputsProps::Trigger(x) => x.input.data_ref.clone(),
            InputsProps::Strategy(x) => x.input.data_ref.clone(),
        }
    }
    pub fn input(&self) -> SettingValueInput {
        match self {
            InputsProps::Trigger(x) => x.input.clone(),
            InputsProps::Strategy(x) => x.input.clone(),
        }
    }
    pub fn duration(&self) -> Option<SettingDurationToggle> {
        match self {
            InputsProps::Trigger(x) => x.duration.clone(),
            InputsProps::Strategy(x) => x.duration.clone(),
        }
    }
}

impl ImplicitClone for InputsProps {}
