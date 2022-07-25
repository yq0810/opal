pub mod t1;
pub use t2::*;

pub mod t2;
pub use t2::*;

use crate::{components::trigger_options::Msg, SettingCallback};

use self::t1::{T1Msg, T1};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriggerConfig {
    pub t1: T1,
    pub t2: T2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msgs {
    T1(T1Msg),
    T2(T2Msg),
}

impl SettingCallback<Msg> for Msgs {
    fn msgFn() -> Box<dyn Fn(Self) -> Msg> {
        let f = |x| -> Msg {
            match x {
                Msgs::T1(x) => Msg::T1OptionUpdate(x),
                Msgs::T2(x) => Msg::T2OptionUpdate(x),
            }
        };
        Box::new(f)
    }
}
