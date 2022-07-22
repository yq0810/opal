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
    pub data_ref: String,
    pub on_change: Box<Callback<String>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SettingDurationToggle {
    pub data_ref: SettingDuration,
    pub on_change: Box<Callback<SettingDuration>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SettingOption<M, T, C>
where
    C: Component + 'static,
    M: Into<C::Message>,
    T: SettingCallback<M> + 'static,
{
    pub input: SettingValueInput,
    pub duration: Option<SettingDurationToggle>,
    phantom: PhantomData<M>,
    phantom2: PhantomData<T>,
    phantom3: PhantomData<C>,
}

impl<M, T, C> SettingOption<M, T, C>
where
    C: Component + 'static,
    M: Into<C::Message>,
    T: SettingCallback<M> + 'static,
{
    pub fn new(
        call_back_input: fn(String) -> T,
        link: &Scope<C>,
        data_ref: String,
        label_text: String,
        call_back_durationAduraion_ref: Option<(fn(SettingDuration) -> T, SettingDuration)>,
    ) -> Self {
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
                SettingOption {
                    input,
                    duration,
                    phantom: PhantomData,
                    phantom2: PhantomData,
                    phantom3: PhantomData,
                }
            }
            None => SettingOption {
                input,
                duration: None,
                phantom: PhantomData,
                phantom2: PhantomData,
                phantom3: PhantomData,
            },
        }
    }
}
