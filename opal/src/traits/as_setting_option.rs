use yew::Component;

use crate::{
    types::setting_option::SettingOption, InputType, SettingActiveToggle, SettingCallbackFn,
    SettingClick, SettingDurationToggle, SettingValueInput, TotalMsgScope, InputValue, label, LabelText, InputSelect, InputDuration, InputClick,
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
        let (label_text_o,input_o, select_o, duration_o, click_o) = from.warp();
        let label_text_o = label_text_o.map(|x|{
            let LabelText(label_text_o) =x;
            label_text_o
        });
        let input = input_o.clone().map(|InputValue(input_msg)| {
            let input_on_change = input_msg.clone().get_pair_link(&link);
            SettingValueInput {
                label_text: label_text_o.clone(),
                msg: input_msg.clone(),
                on_change: input_on_change.clone(),
            }
        });
        let select = select_o.clone().map(|InputSelect(select_msg)| {
            let select_on_change = select_msg.clone().get_pair_link(&link);
            SettingActiveToggle {
                msg: select_msg,
                on_change: select_on_change,
            }
        });

        let duration = duration_o.map(|InputDuration(duration_s, druation_msg)| SettingDurationToggle {
            data_ref: duration_s.clone(),
            on_change: druation_msg.clone().get_pair_link(&link),
        });

        let click = click_o.map(|InputClick(click_msg)| {
            SettingClick {
                label_text: label_text_o.clone(),
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
    }

    fn get_options<M, T, C>(config: &Self::Config, link: TotalMsgScope) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
