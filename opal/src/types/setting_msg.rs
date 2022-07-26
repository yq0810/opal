use yew::{html::Scope, Callback, Component};

use crate::{
    components::{strategy_options, trigger_options::TriggerOptions},
    strategys::{self, StrategyConfig},
    triggers, AsSettingOption, CallbackMsg, InputType, SettingCallback, SettingDuration,
    SettingOption, SettingValueInput,
};

use super::strategys_algo;

#[derive(Clone, PartialEq, Debug)]
pub enum TotalMsg {
    StrategyMsg(strategys::Msgs),
    TriggerMsg(triggers::Msgs),
}
