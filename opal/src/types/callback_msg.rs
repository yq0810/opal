use yew::html::Scope;
use yew::{Callback, Component};

use crate::SettingCallback;

pub trait CallbackMsg {
    type O;
    fn to_callback_fn<M, T, C>(
        call_back_input: fn(String) -> T,
        link: &Scope<C>,
    ) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
    {
        Box::new(link.callback(move |x| T::msgFn()(call_back_input(x))))
    }
    fn as_callback<M, T, C>(&self, link: &Scope<C>) -> Box<Callback<String>>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallback<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
