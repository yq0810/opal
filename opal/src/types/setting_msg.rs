use yew::{html::Scope, Callback, Component};

use crate::{
    area,
    components::{
        coll_card::{self, CollCard},
        strategy_options::{self, StrategyOptions},
        trigger_options::{self, TriggerOptions},
    },
    strategys::{self, StrategyConfig},
    triggers::{self, TriggerConfig},
    AsSettingOption, CallbackMsg, InputType, SettingActiveToggle, SettingDurationToggle,
    SettingOption, SettingValueInput,
};

use super::strategys_algo;

#[derive(Clone, PartialEq, Debug)]
pub enum TotalMsg {
    StrategyMsg(strategys::Msgs),
    TriggerMsg(triggers::Msgs),
    CollCardMsg(area::Msgs),
}
impl TotalMsg {
    pub fn get_pair_link(&self, total_link: &TotalMsgScope) -> Box<Callback<String>> {
        match (self, total_link) {
            (TotalMsg::StrategyMsg(x), TotalMsgScope::StrategyMsgScope(link)) => {
                x.as_callback::<strategy_options::Msg, strategys::Msgs, StrategyOptions>(&link)
            }
            (TotalMsg::TriggerMsg(x), TotalMsgScope::TriggerMsgScope(link)) => {
                x.as_callback::<trigger_options::Msg, triggers::Msgs, TriggerOptions>(&link)
            }
            (TotalMsg::CollCardMsg(x), TotalMsgScope::CollCardMsgScope(link)) => {
                x.as_callback::<coll_card::Msg, area::Msgs, CollCard>(&link)
            }
            _ => panic!("TotalMsg::get_pair_link error"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TotalMsgPage {
    StrategyMsgPage(strategy_options::Msg),
    TriggerMsgPage(trigger_options::Msg),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TotalMsgConfig {
    StrategyMsgConfig(StrategyConfig),
    TriggerMsgConfig(TriggerConfig),
}

pub enum TotalMsgOptions {
    StrategyMsgOptions(StrategyOptions),
    TriggerMsgOptions(TriggerOptions),
}

pub enum TotalMsgScope {
    StrategyMsgScope(Scope<StrategyOptions>),
    TriggerMsgScope(Scope<TriggerOptions>),
    CollCardMsgScope(Scope<CollCard>),
}
