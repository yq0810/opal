use crate::{
    funding_rules::FundingRuleConfig, strategys::StrategyConfig, targets::TargetConfig, AreaConfig,
};

#[derive(Clone, Debug, Default)]
pub struct PickCollConfig {
    pub target_config: TargetConfig,
    pub area: AreaConfig,
    pub strategy: StrategyConfig,
}
