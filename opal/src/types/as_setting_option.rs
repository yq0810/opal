use std::marker::PhantomData;

use yew::{html::Scope, Component};

use crate::{InputType, SettingCallback, SettingOption, SettingValueInput, TotalMsg};

pub trait AsSettingOption {
    type O;
    fn option_input_data<M, T, C>(&self, from: InputType, link: &Scope<C>) -> SettingOption
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
