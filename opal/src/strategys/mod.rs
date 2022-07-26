pub mod volume;
pub use volume::*;

pub mod one;
pub use one::*;

pub mod two;
pub use two::*;
use yew::{html::Scope, Callback, Component};

use crate::{
    components::strategy_options::{self, Msg as SMsg},
    AsSettingOption, CallbackMsg, GetValue, InputType, SettingCallback, SettingOption,
    SettingValueInput, TotalMsg,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StrategyConfig {
    pub s_one: One,
    pub s_two: Two,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msgs {
    One(OneMsg),
    Two(TwoMsg),
}
impl GetValue for Msgs {
    fn get_value(&self) -> String {
        match self {
            Msgs::One(x) => x.get_value(),
            Msgs::Two(x) => x.get_value(),
        }
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
            Msgs::One(m) => todo!(),
            Msgs::Two(m) => todo!(),
        }
    }
}

impl AsSettingOption for Msgs {
    type O = strategy_options::Msg;

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
                    TotalMsg::StrategyMsg(x) => x.as_callback::<M, T, C>(link),
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

impl SettingCallback<SMsg> for Msgs {
    fn msgFn() -> Box<dyn Fn(Self) -> SMsg> {
        let f = |x| -> SMsg {
            match x {
                Msgs::One(x) => SMsg::OneOptionUpdate(x),
                Msgs::Two(x) => SMsg::TwoOptionUpdate(x),
            }
        };
        Box::new(f)
    }
}

// impl<COMP> Into<COMP: Component> for strategy_options::Msg {}
