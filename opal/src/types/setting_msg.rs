use opal_derive::ValueOPMacro;
use yew::{html::Scope, Callback};

use crate::{
    area,
    components::{
        coll_card::{self, CollCard},
        funding_rule_options::{self, FundingRuleOptions},
        strategy_options::{self, StrategyOptions},
        target_options::{self, TargetOptions},
        trigger_options::{self, TriggerOptions},
    },
    funding_rules,
    strategys::{self, StrategyConfig},
    targets,
    triggers::{self, TriggerConfig},
    CallbackMsg,
};

#[derive(Clone, PartialEq, Debug, ValueOPMacro)]
pub enum TotalMsg {
    StrategyMsg(strategys::Msgs),
    TriggerMsg(triggers::Msgs),
    CollCardMsg(area::Msgs),
    TargetMsg(targets::Msgs),
    FundingRuleMsg(funding_rules::Msgs),
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
            (TotalMsg::TargetMsg(x), TotalMsgScope::TargetMsgScope(link)) => {
                x.as_callback::<target_options::Msg, targets::Msgs, TargetOptions>(&link)
            }
            (TotalMsg::FundingRuleMsg(x), TotalMsgScope::FundingRuleMsgScope(link)) => {
                x.as_callback::<funding_rule_options::Msg, funding_rules::Msgs, FundingRuleOptions>(
                    &link,
                )
            }
            _ => panic!("TotalMsg::get_pair_link error"),
        }
    }
}

// impl ValueOP for TotalMsg {
//     fn get_value(&self) -> String {
//         match self {
//             TotalMsg::StrategyMsg(i) => i.get_value(),
//             TotalMsg::TriggerMsg(i) => i.get_value(),
//             TotalMsg::CollCardMsg(i) => i.get_value(),
//             TotalMsg::TargetMsg(i) => todo!(),
//         }
//     }

//     fn set_value(&self, new_value: String) -> Self {
//         todo!()
//     }
// }

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

pub enum TotalMsgScope {
    StrategyMsgScope(Scope<StrategyOptions>),
    TriggerMsgScope(Scope<TriggerOptions>),
    CollCardMsgScope(Scope<CollCard>),
    TargetMsgScope(Scope<TargetOptions>),
    FundingRuleMsgScope(Scope<FundingRuleOptions>),
}
