use std::marker::PhantomData;

use yew::{html::Scope, Component};

use crate::{
    types::setting_option::SettingOption, InputType, SettingActiveToggle, SettingCallbackFn,
    SettingClick, SettingDurationToggle, SettingValueInput, TotalMsg, TotalMsgScope,
};

pub trait AsSettingOption {
    type O;
    type Config;
    fn option_input_data<M, T, C>(from: &InputType, link: &TotalMsgScope) -> SettingOption
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>,
    {
        if let (input_o, select_o, duration_o, click_o) = match from.warp() {
            a @ (Some(_), Some(_), Some(_), None) => a,
            a @ (Some(_), Some(_), None, None) => a,
            a @ (None, None, None, Some(_)) => a,
            _ => todo!(),
        } {
            let input = input_o.clone().map(|input| {
                let (input_s, input_msg) = input;
                let input_on_change = input_msg.clone().get_pair_link(&link);
                SettingValueInput {
                    label_text: input_s.to_string(),
                    msg: input_msg.clone(),
                    on_change: input_on_change.clone(),
                }
            });
            let select = select_o.clone().map(|select_msg| {
                let select_on_change = select_msg.clone().get_pair_link(&link);
                SettingActiveToggle {
                    msg: select_msg,
                    on_change: select_on_change,
                }
            });

            let duration = duration_o.map(|(duration_s, druation_msg)| SettingDurationToggle {
                data_ref: duration_s.clone(),
                on_change: druation_msg.clone().get_pair_link(&link),
            });

            let click = click_o.map(|click| {
                let (label_text, click_msg) = click;
                SettingClick {
                    label_text: label_text.to_string().clone(),
                    msg: click_msg.clone(),
                    on_click: click_msg.clone().get_pair_link(&link),
                }
            });
            SettingOption {
                select,
                input,
                duration,
                click,
            }
        } else {
            todo!()
        }
    }

    fn get_options<M, T, C>(config: &Self::Config, link: TotalMsgScope) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
