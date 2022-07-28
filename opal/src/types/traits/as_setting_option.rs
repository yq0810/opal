use std::marker::PhantomData;

use yew::{html::Scope, Component};

use crate::{
    types::setting_option::SettingOption, InputType, SettingActiveToggle, SettingCallbackFn,
    SettingDurationToggle, SettingValueInput, TotalMsg, TotalMsgScope,
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
        if let (Some(input), Some(select), duration) = match from.warp() {
            a @ (Some(_), Some(_), Some(_)) => a,
            a @ (Some(_), Some(_), None) => a,
            _ => todo!(),
        } {
            let (input_s, input_msg) = input;
            let select_msg = select;
            let input_on_change = input_msg.clone().get_pair_link(&link);

            let duration = duration.map(|(duration_s, druation_msg)| SettingDurationToggle {
                data_ref: duration_s.clone(),
                on_change: druation_msg.clone().get_pair_link(&link),
            });
            let select_on_change = select_msg.clone().get_pair_link(&link);
            SettingOption {
                input: SettingValueInput {
                    label_text: input_s.to_string(),
                    msg: input_msg.clone(),
                    on_change: input_on_change.clone(),
                },
                duration,
                select: SettingActiveToggle {
                    msg: select_msg,
                    on_change: select_on_change,
                },
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
