use crate::{funding_rules::FundingRuleConfig, targets::TargetConfig, AreaConfig};

#[derive(Clone, Debug, Default)]
pub struct PickCollConfig {
    pub target_config: TargetConfig,
    pub area: AreaConfig,
}
