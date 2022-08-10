use self::{
    funding_rules::FundingRuleConfig, strategys::StrategyConfig, targets::TargetConfig,
    triggers::TriggerConfig,
};

pub mod funding_rules;
pub mod strategys;
pub mod targets;
pub mod triggers;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct SettingCardConfig {
    pub strategy: StrategyConfig,
    pub trigger: TriggerConfig,
    pub target: TargetConfig,
    pub funding_rule: FundingRuleConfig,
    pub active_tab: u32,
}
