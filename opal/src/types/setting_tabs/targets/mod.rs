use self::full::{Full, FullMsg};
use crate::components;
use opal_derive::{AsSettingOptionMacro, CallbackMsgMacro, SettingCallbackFnMacro, ValueOPMacro};
pub mod full;
use crate::SettingCallbackFn;
pub use full::*;

#[derive(Clone, Debug, PartialEq, ValueOPMacro, SettingCallbackFnMacro, CallbackMsgMacro)]
#[page("target_options")]
pub enum Msgs {
    Full(FullMsg),
}

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("target_options")]
pub struct TargetConfig {
    full: Full,
}
