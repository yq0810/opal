use crate::{
    components::{
        strategy_options::{self, Msg, StrategyOptions},
        trigger_options::{self, TriggerOptions},
    },
    func_components::*,
    strategys::{self, Msgs, OneMsg},
    triggers::{self, T2Msg},
    SettingCallbackFn, SettingDuration, SettingDurationToggle, SettingOption, SettingValueInput,
    ValueOP,
};
use log::debug;
use web_sys::{HtmlInputElement, KeyboardEvent, Node};
use yew::html::IntoPropValue;
use yew::{
    classes, function_component, functional::*, html, html::ImplicitClone, Callback, NodeRef,
    Properties,
};

#[derive(Clone, PartialEq, Properties)]
pub struct StrategyInputProps {
    // pub label_text:String,
    // pub node_ref: NodeRef,
    // pub on_change: Box<Callback<String>>,
    pub option: SettingOption,
}

impl ImplicitClone for SettingOption {}

#[function_component(SettingInput)]
pub fn strategy_input(props: &StrategyInputProps) -> Html {
    let search_bar_classes = classes!(
        "dark:bg-slate-900",
        "bg-white",
        "flex",
        "rounded-md",
        "overflow-hidden",
        "w-full",
        "pl-2",
        "mb-4",
        "drop-shadow-light",
    );
    let input_classes = "focus:outline-none rounded-none rounded bg-gray-50 border border-gray-300 text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 w-12 text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
    let input_classes_duration = "w-20 focus:outline-none rounded-none rounded-r-lg bg-gray-50 border border-gray-300 text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
    let toggle_classes = classes!(
        "h-10",
        "w-35",
        "p-2",
        "rounded-md",
        "dark:text-slate-50",
        "self-center",
        "font-input",
        "text-sm",
        "text-center",
    );
    let toggle_classes_o = classes!(
        "h-10",
        "w-30",
        "rounded-md",
        "hover:bg-slate-100",
        "hover:dark:bg-slate-600",
        "dark:text-slate-50",
        "self-center",
        "font-input",
        "text-sm",
        "text-center",
    );

    let text_empty = use_state_eq(|| true);
    let input_ref = NodeRef::default();
    let input_duration_value_ref = NodeRef::default();
    let input_ref = input_ref.clone();
    let option = props.option.clone();
    let input_ref_2 = input_ref.clone();
    let select_input_ref = NodeRef::default();
    let select_input_ref_2 = select_input_ref.clone();
    let init = {
        let on_search = option.input.on_change.clone();
        let value = match option.input.msg.clone() {
            crate::TotalMsg::StrategyMsg(x) => x.get_value(),
            crate::TotalMsg::TriggerMsg(x) => x.get_value(),
        };
        use_effect(move || {
            if let Some(input) = input_ref_2.cast::<HtmlInputElement>() {
                input.set_value(&value);
            }
            || ()
        });
        let value = match option.select.msg.clone() {
            crate::TotalMsg::StrategyMsg(x) => x.get_value().parse().unwrap(),
            crate::TotalMsg::TriggerMsg(x) => x.get_value().parse().unwrap(),
        };
        use_effect(move || {
            if let Some(input) = select_input_ref_2.cast::<HtmlInputElement>() {
                input.set_checked(value);
            }
            || ()
        });
        // on_search.emit(value);
    };
    let duration_ref_o = option
        .duration
        .clone()
        .and_then(|x| Some(x.data_ref.clone()));
    match duration_ref_o {
        Some(x) => {
            let input_duration_value_ref = input_duration_value_ref.clone();
            use_effect(move || {
                if let Some(input) = input_duration_value_ref.cast::<HtmlInputElement>() {
                    // let _ = input.focus();
                    input.set_value(&x.get_value().to_string());
                }
                || ()
            });
        }
        None => (),
    }

    let oninput_duration_value = {
        let input_ref = input_duration_value_ref.clone();
        let duration_o = option.clone().duration;
        let duration = match duration_o {
            Some(duration) => Some(move |_e| {
                let s = input_ref
                    .cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default();
                let nx = s.parse::<i32>().unwrap_or_default();
                let back = duration.data_ref.set_value(nx);
                duration.on_change.emit(back.to_string())
            }),
            None => None,
        };
        duration
    };

    let oninput = {
        let text_empty = text_empty.clone();
        let input_ref = input_ref.clone();
        let on_search = option.input.on_change.clone();
        move |_e| {
            let s = input_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();
            text_empty.set(s.is_empty());
            on_search.emit(s)
        }
    };
    let toggle_onclick = {
        let duration_o = option.clone().duration;
        match duration_o {
            Some(duration) => Some(move |_e| {
                let back = match duration.data_ref {
                    crate::SettingDuration::Days(x) => SettingDuration::Hours(x),
                    crate::SettingDuration::Hours(x) => SettingDuration::Days(x),
                };
                duration.on_change.emit(back.to_string())
            }),
            None => None,
        }
    };
    let (is_d, time_duration) = match option.duration.clone() {
        Some(d) => (true, d.data_ref.Display()),
        None => (false, "".to_string()),
    };

    let select_onchange = {
        let option = option.clone();
        let on_search = option.select.on_change.clone();
        let select_input_ref = select_input_ref.clone();
        move |_e| {
            let s = select_input_ref
                .cast::<HtmlInputElement>()
                .map(|select| select.checked())
                .unwrap_or_default();
            on_search.emit(s.to_string())
        }
    };

    html! {
        <div key={option.input.label_text.clone()}>
            <div class={search_bar_classes}>
                <input type="checkbox" class="px-3"
                    ref={select_input_ref}
                    onclick={select_onchange}
                  />
                <button class={toggle_classes.clone()} style="" disabled=true >{props.option.input.label_text.clone()}</button>
                <input class={input_classes} type="text" ref={input_ref} {oninput} />
                {
                    if is_d {
                        let oninput_duration_value = oninput_duration_value.clone().unwrap();
                        let toggle_onclick = toggle_onclick.clone().unwrap();
                        html! {
                            <div class="flex">
                                <button class={toggle_classes_o} onclick={toggle_onclick}>
                                    {time_duration}
                                </button>
                                <input class={input_classes_duration} type="text" ref={input_duration_value_ref} oninput={oninput_duration_value}/>
                            </div>
                        }
                    } else {
                        html! {}
                    }

                }
            </div>
        </div>
    }
}
