pub mod t1;
use opal_derive::ValueOPMacro;
pub use t2::*;

pub mod t2;
pub use t2::*;
use yew::{html::Scope, Callback, Component};

use crate::{
    components::{
        strategy_options,
        trigger_options::{self, Msg as TMsg},
    },
    AsInputType, AsSettingOption, CallbackMsg, InputType, SettingActiveToggle, SettingCallbackFn,
    SettingOption, SettingValueInput, TotalMsg, TotalMsgScope, ValueOP,
};

use self::t1::{T1Msg, T1};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriggerConfig {
    pub t1: T1,
    pub t2: T2,
}

#[derive(Clone, Debug, PartialEq, ValueOPMacro)]
pub enum Msgs {
    T1(T1Msg),
    T2(T2Msg),
}

impl AsSettingOption for Msgs {
    type O = trigger_options::Msg;
    type Config = TriggerConfig;
    fn get_options<M, T, C>(config: &Self::Config, link: TotalMsgScope) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        let a = config.t1.input_type();
        let b = config.t2.input_type();
        let l = vec![a, b];
        l.iter()
            .map(|input_type| -> SettingOption {
                Self::option_input_data::<M, T, C>(input_type.clone(), &link)
            })
            .collect()
    }
}

impl CallbackMsg for Msgs {
    type O = trigger_options::Msg;

    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        match self {
            Msgs::T1(m) => match m {
                T1Msg::UpdatePercentage(_) => Self::to_callback_fn(
                    |x| Msgs::T1(T1Msg::UpdatePercentage(x.parse().ok())),
                    link,
                ),
                T1Msg::UpdateActive(_) => {
                    Self::to_callback_fn(|x| Msgs::T1(T1Msg::UpdateActive(x.parse().ok())), link)
                }
            },
            Msgs::T2(m) => match m {
                T2Msg::UpdatePercentage(_) => Self::to_callback_fn(
                    |x| Msgs::T2(T2Msg::UpdatePercentage(x.parse().ok())),
                    link,
                ),
                T2Msg::UpdateActive(_) => {
                    Self::to_callback_fn(|x| Msgs::T2(T2Msg::UpdateActive(x.parse().ok())), link)
                }
            },
        }
    }
}

impl SettingCallbackFn<TMsg> for Msgs {
    fn msgFn() -> Box<dyn Fn(Self) -> TMsg> {
        let f = |x| -> TMsg {
            match x {
                Msgs::T1(x) => TMsg::T1OptionUpdate(x),
                Msgs::T2(x) => TMsg::T2OptionUpdate(x),
            }
        };
        Box::new(f)
    }
}
