pub mod volume;

use opal_derive::{AsSettingOptionMacro, OptionMsg};
pub use volume::*;

pub mod one;
pub use one::*;

pub mod total_volume;
pub use total_volume::*;
pub mod tx_count;
pub use tx_count::*;

use crate::{components, SettingCallbackFn};

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("strategy_options")]
pub struct StrategyConfig {
    pub s_one: One,
    pub total_volume: TotalVolume,
    pub tx_count: TxCount,
}

#[derive(Clone, Debug, PartialEq, OptionMsg)]
#[page("strategy_options")]
pub enum Msgs {
    One(OneMsg),
    TotalVolume(TotalVolumeMsg),
    TxCount(TxCountMsg),
}
