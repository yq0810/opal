use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use chrono::Duration;
use web_sys::Node;
use yew::html::ImplicitClone;
use yew::{html::Scope, NodeRef};
use yew::{Callback, Component};

use crate::components::setting_card::SettingCard;
use crate::components::strategy_options::StrategyOptions;
use crate::pages::{Index, Msg};
use crate::{CallbackMsg, InputType, TotalMsg};

pub trait SettingCallback<M> {
    fn msgFn() -> Box<dyn Fn(Self) -> M>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingDuration {
    Days(i32),
    Hours(i32),
}

impl SettingDuration {
    pub fn set_value(&self, nv: i32) -> Self {
        match &self {
            SettingDuration::Days(_) => SettingDuration::Days(nv),
            SettingDuration::Hours(_) => SettingDuration::Hours(nv),
        }
    }
    pub fn get_value(&self) -> i32 {
        match &self {
            SettingDuration::Days(x) | SettingDuration::Hours(x) => x.clone(),
        }
    }
    pub fn to_duration(&self) -> Duration {
        match &self {
            SettingDuration::Days(x) => Duration::days(*x as i64),
            SettingDuration::Hours(x) => Duration::hours(*x as i64),
        }
    }
}

impl Default for SettingDuration {
    fn default() -> Self {
        Self::Hours(1)
    }
}

impl SettingDuration {
    pub fn Display(&self) -> String {
        match &self {
            SettingDuration::Days(_) => "/Days".to_string(),
            SettingDuration::Hours(_) => "/Hours".to_string(),
        }
    }
    pub fn Value(&self) -> i32 {
        match &self {
            SettingDuration::Days(x) => x.clone(),
            SettingDuration::Hours(x) => x.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SettingValueInput {
    pub label_text: String,
    pub msg: TotalMsg,
    pub on_change: Box<Callback<String>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SettingDurationToggle {
    pub data_ref: SettingDuration,
    pub on_change: Box<Callback<SettingDuration>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SettingOption {
    pub input: SettingValueInput,
    pub duration: Option<SettingDurationToggle>,
}
