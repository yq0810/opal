use yew::html::Scope;
use yew::{Callback, Component};

use crate::SettingCallbackFn;

pub trait CallbackMsg {
    type O;
    fn to_callback_fn<M, T, C>(
        call_back_input: fn(String) -> T,
        link: &Scope<C>,
    ) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
    {
        Box::new(link.callback(move |x| T::msg_fn()(call_back_input(x))))
    }
    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
