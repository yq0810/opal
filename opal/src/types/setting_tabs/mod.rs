use self::{strategys::StrategyConfig, triggers::TriggerConfig};

pub mod strategys;
pub mod triggers;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct SettingCardConfig {
    pub strategy: StrategyConfig,
    pub trigger: TriggerConfig,
}
