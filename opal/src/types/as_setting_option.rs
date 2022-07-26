use std::marker::PhantomData;

use yew::{html::Scope, Component};

use crate::{InputType, SettingCallback, SettingOption, SettingValueInput, TotalMsg};

pub trait AsSettingOption {
    type O;
    type Config;
    fn option_input_data<M, T, C>(from: &InputType, link: &Scope<C>) -> SettingOption
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;

    fn get_options<M, T, C>(config: &Self::Config, link: &Scope<C>) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
