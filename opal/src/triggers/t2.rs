use std::default;

use chrono::{DateTime, Duration, Utc};

use crate::{components::trigger_options::Msg, SettingCallback, SettingDuration};

#[derive(Clone, Debug, PartialEq)]
pub enum T2Msg {
    UpdatePercentage(Option<f64>),
    UpdateActive(bool),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct T2 {
    pub percentage: u32,
    pub active: bool,
}
