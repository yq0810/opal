pub mod volume;

use opal_derive::{AsSettingOptionMacro, OptionMsg};
pub use volume::*;

pub mod one;
pub use one::*;

pub mod two;
pub use two::*;
pub mod three;
pub use three::*;

use crate::{components, SettingCallbackFn};

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("strategy_options")]
pub struct StrategyConfig {
    pub s_one: One,
    pub s_two: Two,
    pub s_three: Three,
}

#[derive(Clone, Debug, PartialEq, OptionMsg)]
#[page("strategy_options")]
pub enum Msgs {
    One(OneMsg),
    Two(TwoMsg),
    Three(ThreeMsg),
}
