use self::{strategys::StrategyConfig, targets::TargetConfig, triggers::TriggerConfig};

pub mod strategys;
pub mod targets;
pub mod triggers;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct SettingCardConfig {
    pub strategy: StrategyConfig,
    pub trigger: TriggerConfig,
    pub target: TargetConfig,
    pub active_tab: u32,
}
