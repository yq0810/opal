pub mod t1;
use opal_derive::{AsSettingOptionMacro, CallbackMsgMacro, SettingCallbackFnMacro, ValueOPMacro};
pub use t2::*;

pub mod t2;
use self::t1::{T1Msg, T1};
use crate::components;
use crate::SettingCallbackFn;
pub use t2::*;

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("trigger_options")]
pub struct TriggerConfig {
    pub t1: T1,
    pub t2: T2,
}

#[derive(Clone, Debug, PartialEq, ValueOPMacro, SettingCallbackFnMacro, CallbackMsgMacro)]
#[page("trigger_options")]
pub enum Msgs {
    T1(T1Msg),
    T2(T2Msg),
}
