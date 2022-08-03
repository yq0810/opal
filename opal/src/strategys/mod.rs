pub mod volume;
use std::{num::ParseIntError, str::FromStr};

use futures::future::SelectAll;
use opal_derive::ValueOPMacro;
pub use volume::*;

pub mod one;
pub use one::*;

pub mod two;
pub use two::*;
pub mod three;
pub use three::*;
use yew::{html::Scope, Callback, Component};

use crate::{
    components::strategy_options::{self, Msg as SMsg},
    AsInputType, AsSettingOption, CallbackMsg, InputType, SettingActiveToggle, SettingCallbackFn,
    SettingDuration, SettingDurationToggle, SettingOption, SettingValueInput, TotalMsg,
    TotalMsgScope, ValueOP,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StrategyConfig {
    pub s_one: One,
    pub s_two: Two,
    pub s_three: Three,
}

#[derive(Clone, Debug, PartialEq, ValueOPMacro)]
pub enum Msgs {
    One(OneMsg),
    Two(TwoMsg),
    Three(ThreeMsg),
}

impl CallbackMsg for Msgs {
    type O = strategy_options::Msg;

    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        match self {
            Msgs::One(msg) => match msg {
                OneMsg::UpdateVolumeRateValue(_) => Self::to_callback_fn(
                    |x| Msgs::One(OneMsg::UpdateVolumeRateValue(x.parse().ok())),
                    link,
                ),
                OneMsg::UpdateVolumeRateDuration(_) => Self::to_callback_fn(
                    |x| Msgs::One(OneMsg::UpdateVolumeRateDuration(x.parse().ok())),
                    link,
                ),
                OneMsg::UpdateVolumeRateSelect(_) => Self::to_callback_fn(
                    |x| Msgs::One(OneMsg::UpdateVolumeRateSelect(x.parse().ok())),
                    link,
                ),
            },
            Msgs::Two(m) => match m {
                TwoMsg::UpdateVolumeTotalValue(_) => Self::to_callback_fn(
                    |x| Msgs::Two(TwoMsg::UpdateVolumeTotalValue(x.parse().ok())),
                    link,
                ),
                TwoMsg::UpdateVolumeTotalSelect(_) => Self::to_callback_fn(
                    |x| Msgs::Two(TwoMsg::UpdateVolumeTotalSelect(x.parse().ok())),
                    link,
                ),
            },
            Msgs::Three(m) => match m {
                ThreeMsg::UpdateTxCountValue(_) => Self::to_callback_fn(
                    |x| Msgs::Three(ThreeMsg::UpdateTxCountValue(x.parse().ok())),
                    link,
                ),
                ThreeMsg::UpdateTxCountDuration(_) => Self::to_callback_fn(
                    |x| Msgs::Three(ThreeMsg::UpdateTxCountDuration(x.parse().ok())),
                    link,
                ),
                ThreeMsg::UpdateTxCountSelect(_) => Self::to_callback_fn(
                    |x| Msgs::Three(ThreeMsg::UpdateTxCountSelect(x.parse().ok())),
                    link,
                ),
            },
        }
    }
}

impl AsSettingOption for Msgs {
    type O = strategy_options::Msg;
    type Config = StrategyConfig;

    fn get_options<M, T, C>(config: &Self::Config, link: TotalMsgScope) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        let a = config.s_one.input_type();
        let b = config.s_two.input_type();
        let c = config.s_three.input_type();
        let l = vec![a, b, c];
        l.iter()
            .map(|input_type| -> SettingOption {
                Self::option_input_data::<M, T, C>(input_type, &link)
            })
            .collect()
    }
}

impl SettingCallbackFn<SMsg> for Msgs {
    fn msgFn() -> Box<dyn Fn(Self) -> SMsg> {
        let f = |x| -> SMsg {
            match x {
                Msgs::One(x) => SMsg::OneOptionUpdate(x),
                Msgs::Two(x) => SMsg::TwoOptionUpdate(x),
                Msgs::Three(x) => SMsg::ThreeOptionUpdate(x),
            }
        };
        Box::new(f)
    }
}

// impl<COMP> Into<COMP: Component> for strategy_options::Msg {}
