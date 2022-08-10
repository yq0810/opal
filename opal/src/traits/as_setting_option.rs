use yew::Component;

use crate::{
    label, types::setting_option::SettingOption, InputClick, InputDuration, InputMultiSelect,
    InputSelect, InputType, InputValue, LabelText, SettingCallbackFn, SettingClick,
    SettingDurationToggle, SettingSelect, SettingValueInput, TotalMsgScope,
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
        let (label_text_o, input_o, select_o, duration_o, click_o, multi_select_o) = from.warp();
        let label_text_o = label_text_o.map(|x| {
            let LabelText(label_text_o) = x;
            label_text_o
        });
        let click = click_o.map(|InputClick(click_msg)| SettingClick {
            label_text: label_text_o.clone(),
            msg: click_msg.clone(),
            on_click: click_msg.clone().get_pair_link(&link),
        });
        let input = input_o.clone().map(|InputValue(input_msg)| {
            let input_on_change = input_msg.clone().get_pair_link(&link);
            SettingValueInput {
                label_text: None,
                msg: input_msg.clone(),
                on_change: input_on_change.clone(),
            }
        });
        let select = select_o.clone().map(|InputSelect(select_msg)| {
            let select_on_change = select_msg.clone().get_pair_link(&link);
            SettingSelect {
                msg: select_msg,
                on_change: select_on_change,
            }
        });

        let duration =
            duration_o.map(
                |InputDuration(duration_s, druation_msg)| SettingDurationToggle {
                    data_ref: duration_s.clone(),
                    on_change: druation_msg.clone().get_pair_link(&link),
                },
            );
        let multi_select = multi_select_o.map(|InputMultiSelect(select_msg)| {
            select_msg
                .iter()
                .cloned()
                .flat_map(|setting_select| {
                    let on_change = setting_select.clone().get_pair_link(&link);
                    let msg = setting_select.clone();
                    vec![SettingSelect { msg, on_change }]
                })
                .collect::<Vec<_>>()
        });

        SettingOption {
            label_text: if click.is_some() { None } else { label_text_o },
            select,
            input,
            duration,
            click,
            multi_select,
        }
    }

    fn get_options<M, T, C>(config: &Self::Config, link: TotalMsgScope) -> Vec<SettingOption>
    where
        C: Component + 'static,
        M: Into<C::Message>,
        T: SettingCallbackFn<M> + 'static,
        <C as yew::Component>::Message: From<Self::O>;
}
