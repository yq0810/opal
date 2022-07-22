use std::cell::RefCell;
use std::rc::Rc;

use chrono::Duration;
use web_sys::Node;
use yew::html::ImplicitClone;
use yew::Callback;
use yew::{html::Scope, NodeRef};

use crate::components::setting_card::SettingCard;
use crate::pages::{Index, Msg};
use crate::strategys::Strategy;

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
    pub data_ref: String,
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
impl ImplicitClone for SettingOption {}

impl SettingOption {
    pub fn new<T>(
        call_back_input: fn(String) -> T,
        link: &Scope<SettingCard>,
        data_ref: String,
        label_text: String,
        call_back_durationAduraion_ref: Option<(fn(SettingDuration) -> T, SettingDuration)>,
    ) -> Self
    where
        T: Strategy + 'static,
    {
        let input = SettingValueInput {
            label_text,
            on_change: Box::new(link.callback(move |x| T::msgFn()(call_back_input(x)))),
            data_ref,
        };
        match call_back_durationAduraion_ref {
            Some((call_back_duration, data_ref_duration)) => {
                let duration = Some(SettingDurationToggle {
                    on_change: Box::new(link.callback(move |x| T::msgFn()(call_back_duration(x)))),
                    data_ref: data_ref_duration,
                });
                SettingOption { input, duration }
            }
            None => SettingOption {
                input,
                duration: None,
            },
        }
    }
}
