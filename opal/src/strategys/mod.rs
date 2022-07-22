pub mod volume;
pub use volume::*;

pub mod one;
pub use one::*;

pub mod two;
pub use two::*;

use crate::{components::strategy_options::Msg, SettingCallback};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StrategyConfig {
    pub s_one: One,
    pub s_two: Two,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msgs {
    One(OneMsg),
    Two(TwoMsg),
}

impl SettingCallback<Msg> for Msgs {
    fn msgFn() -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg {
            match x {
                Msgs::One(x) => Msg::OneOptionUpdate(x),
                Msgs::Two(x) => Msg::TwoOptionUpdate(x),
            }
        };
        Box::new(f)
    }
}
