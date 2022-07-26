pub mod t1;
pub use t2::*;

pub mod t2;
pub use t2::*;
use yew::{html::Scope, Callback, Component};

use crate::{
    components::{
        strategy_options,
        trigger_options::{self, Msg as TMsg},
    },
    AsSettingOption, CallbackMsg, GetValue, InputType, SettingCallback, SettingOption,
    SettingValueInput, TotalMsg,
};

use self::t1::{T1Msg, T1};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TriggerConfig {
    pub t1: T1,
    pub t2: T2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msgs {
    T1(T1Msg),
    T2(T2Msg),
}

impl GetValue for Msgs {
    fn get_value(&self) -> String {
        match self {
            Msgs::T1(x) => x.get_value(),
            Msgs::T2(x) => x.get_value(),
        }
    }
}

impl AsSettingOption for Msgs {
    type O = trigger_options::Msg;

    fn option_input_data<M, T, C>(&self, from: InputType, link: &Scope<C>) -> SettingOption
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        let (label_text, msg, on_change) = match from {
            InputType::SelectValue((label_text, msg), x) => {
                let on_change = match msg {
                    TotalMsg::TriggerMsg(x) => x.as_callback::<M, T, C>(link),
                    _ => panic!("unexpected msg"),
                };
                (label_text.to_string(), msg, on_change)
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
}

impl CallbackMsg for Msgs {
    type O = trigger_options::Msg;
    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        match self {
            Msgs::T1(m) => {
                match m {
                    T1Msg::UpdatePercentage(_) => Self::to_callback_fn(
                        |x| Msgs::T1(T1Msg::UpdatePercentage(x.parse().ok())),
                        link,
                    ),
                    T1Msg::UpdateActive(_) => todo!(), // T1Msg::UpdateActive(_) => |x| T1Msg::UpdateActive(x.parse().ok()),
                }
            }
            Msgs::T2(m) => todo!(),
        }
    }
}

impl SettingCallback<TMsg> for Msgs {
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
