use std::cell::RefCell;
use std::rc::Rc;

use chrono::Duration;
use web_sys::Node;
use yew::html::ImplicitClone;
use yew::{NodeRef, html::Scope};
use yew::Callback;

use crate::app::{Msg, App};
use crate::strategys::Strategy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingDuration {
    Days(i32),
    Hours(i32)
}

impl Default for SettingDuration {
    fn default() -> Self {
        Self::Hours(1)
    }
}

impl SettingDuration {
    pub fn Display(&self) -> String {
        match &self {
            SettingDuration::Days(_) => "Days".to_string(),
            SettingDuration::Hours(_) => "Hours".to_string(),
        }
    }
    pub fn Value(&self) -> i32 {
        match &self {
            SettingDuration::Days(x) => x.clone(),
            SettingDuration::Hours(x) => x.clone(),
        }
    }
}

#[derive(Clone,PartialEq, Debug)]
pub struct SettingValueInput {
    pub label_text:String,
    pub node_ref: NodeRef,
    pub data_ref: String,
    pub on_change: Box<Callback<String>>,
}

#[derive(Clone,PartialEq, Debug)]
pub struct SettingDurationToggle {
    pub node_ref: NodeRef,
    pub data_ref: SettingDuration,
    pub on_change: Box<Callback<SettingDuration>>,
}

#[derive(Clone,PartialEq, Debug)]
pub struct SettingOption {
    pub input:SettingValueInput,
    pub duration:Option<SettingDurationToggle>
}
impl ImplicitClone for SettingOption {

}

impl SettingOption {
    pub fn new<T>(
               call_back_input:fn(String) -> T,
               link:&Scope<App>,
               data_ref: String,
               label_text:String,
               call_back_duration:fn(SettingDuration) -> T,
               duraion_ref: SettingDuration,
            ) -> Self 
            where T: Strategy + 'static
        {
        let input = SettingValueInput { 
            label_text,
            node_ref: NodeRef::default(),
            on_change: Box::new(link.callback(move |x| T::msgFn()(call_back_input(x)))),
            data_ref
        };
        let duration = SettingDurationToggle { 
            node_ref: NodeRef::default(),
            on_change: Box::new(link.callback(move |x| T::msgFn()(call_back_duration(x)))),
            data_ref: duraion_ref,
        };
        Self { 
            input, duration: Some(duration)
        }
        

    }
}