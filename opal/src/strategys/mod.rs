pub mod volume;
use std::{num::ParseIntError, str::FromStr};

use futures::future::SelectAll;
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
    AsSettingOption, CallbackMsg, GetValue, InputType, InputTypeExt, SettingCallback,
    SettingDuration, SettingOption, SettingValueInput, TotalMsg,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StrategyConfig {
    pub s_one: One,
    pub s_two: Two,
    pub s_three: Three,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msgs {
    One(OneMsg),
    Two(TwoMsg),
    Three(ThreeMsg),
}
impl GetValue for Msgs {
    fn get_value(&self) -> String {
        match self {
            Msgs::One(x) => x.get_value(),
            Msgs::Two(x) => x.get_value(),
            Msgs::Three(x) => x.get_value(),
        }
    }

    fn to_total_msg(&self) -> TotalMsg {
        match self {
            Msgs::One(x) => x.to_total_msg(),
            Msgs::Two(x) => x.to_total_msg(),
            Msgs::Three(x) => x.to_total_msg(),
        }
    }
}

impl FromStr for SettingDuration {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SettingDuration::Days(55))
    }
}

impl CallbackMsg for Msgs {
    type O = strategy_options::Msg;

    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
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
    fn option_input_data<M, T, C>(from: &InputType, link: &Scope<C>) -> SettingOption
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        let (label_text, msg, on_change) = match from {
            InputType::SelectValue((label_text, msg), x) => {
                let on_change = match msg.clone() {
                    TotalMsg::StrategyMsg(x) => x.as_callback::<M, T, C>(link),
                    _ => panic!("unexpected msg"),
                };
                (label_text.to_string(), msg.clone(), on_change)
            }
            InputType::SelectValueDuration(_, _, _) => todo!(),
        };
        SettingOption {
            input: SettingValueInput {
                label_text,
                msg,
                on_change,
            },
            duration: None,
        }
    }

    fn get_options<M, T, C>(config: &Self::Config, link: &Scope<C>) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        let a = config.s_one.input_type();
        let b = config.s_two.input_type();
        let c = config.s_three.input_type();
        let l = vec![a, b, c];
        l.iter()
            .map(|input_type| match input_type {
                InputType::SelectValue((_, x), _) => {
                    Self::option_input_data::<M, T, C>(input_type.clone(), link)
                }
                InputType::SelectValueDuration(_, _, _) => //todo ,
            })
            .collect()
    }
}

impl SettingCallback<SMsg> for Msgs {
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
