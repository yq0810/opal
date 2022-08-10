use crate::{SettingDuration, SettingOption, ValueOP};
use log::debug;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{
    classes, function_component, functional::*, html, html::ImplicitClone, Html, NodeRef,
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
    let _ = {
        option.clone().input.map(|input| {
            let value = match input.msg.clone() {
                crate::TotalMsg::StrategyMsg(x) => x.get_value(),
                crate::TotalMsg::TriggerMsg(x) => x.get_value(),
                crate::TotalMsg::CollCardMsg(x) => x.get_value(),
                crate::TotalMsg::TargetMsg(x) => x.get_value(),
                crate::TotalMsg::FundingRuleMsg(x) => x.get_value(),
            };
            use_effect(move || {
                if let Some(input) = input_ref_2.cast::<HtmlInputElement>() {
                    input.set_value(&value);
                }
                || ()
            });
        });

        option.clone().select.map(|x| {
            let v = match x.msg.clone() {
                crate::TotalMsg::StrategyMsg(x) => x.get_value().parse().unwrap(),
                crate::TotalMsg::TriggerMsg(x) => x.get_value().parse().unwrap(),
                crate::TotalMsg::CollCardMsg(x) => x.get_value().parse().unwrap(),
                crate::TotalMsg::TargetMsg(x) => x.get_value().parse().unwrap(),
                crate::TotalMsg::FundingRuleMsg(x) => x.get_value().parse().unwrap(),
            };
            use_effect(move || {
                if let Some(input) = select_input_ref_2.cast::<HtmlInputElement>() {
                    input.set_checked(v);
                }
                || ()
            });
        });
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
    let label_text = option.label_text.clone().map(|label_text| {
        html! {
            <div class={toggle_classes.clone()}>{label_text}</div>
        }
    });

    let input = option.input.clone().map(|input| {
        let text_empty = text_empty.clone();
        let input_ref_copy = input_ref.clone();
        let input_ref_copy_2 = input_ref.clone();
        let on_search = input.on_change.clone();
        let on_search_2 = input.on_change.clone();
        let oninput = move |_e| {
            let s = input_ref_copy
                .cast::<HtmlInputElement>()
                .map(|input| input.value())
                .unwrap_or_default();
            text_empty.set(s.is_empty());
            on_search.emit(s)
        };
        let onkeypress = {
            move |e: KeyboardEvent| {
                if e.key() == "Enter" {
                    let s = input_ref_copy_2
                        .cast::<HtmlInputElement>()
                        .map(|input| input.value())
                        .unwrap_or_default();
                    if !s.is_empty() {
                        on_search_2.emit(s + " ");
                    }
                }
            }
        };
        html! {
            <>
            <div class={toggle_classes.clone()}>{input.label_text.unwrap_or_default()}</div>
            <input class={input_classes} type="text" ref={input_ref} {oninput} {onkeypress} />
            </>
        }
    });
    let duration = option.duration.clone().map(|duration| {
        let toggle_classes_o_copy = toggle_classes_o.clone();
        let display = duration.data_ref.display();
        let event = move |_e| {
            let back = match duration.data_ref {
                crate::SettingDuration::Days(x) => SettingDuration::Hours(x),
                crate::SettingDuration::Hours(x) => SettingDuration::Days(x),
            };
            duration.on_change.emit(back.to_string())
        };
        html! {
            <div class="flex">
                <button class={toggle_classes_o_copy} onclick={event}>
                    {display}
                </button>
                <input class={input_classes_duration} type="text" ref={input_duration_value_ref} oninput={oninput_duration_value}/>
            </div>
        }
    });

    let select = option.select.clone().map(|select| {
        let on_search = select.on_change.clone();
        let select_input_ref_copy = select_input_ref.clone();
        let event = move |_e| {
            let s = select_input_ref_copy
                .cast::<HtmlInputElement>()
                .map(|select| select.checked())
                .unwrap_or_default();
            on_search.emit(s.to_string())
        };
        html! {
            <input type="checkbox" class="px-3"
                ref={select_input_ref}
                onclick={event}
                />
        }
    });

    let click = option.click.clone().map(|click| {
        let toggle_classes_o_copy = toggle_classes_o.clone();
        let on_click = click.on_click.clone();
        let value = match click.msg.clone() {
            crate::TotalMsg::CollCardMsg(x) => x.get_value(),
            _ => panic!("not coll card"),
        };
        let event = move |_e| on_click.emit(value.clone());
        html! {
            <button class={toggle_classes_o_copy} onclick={event}>
                {click.label_text.unwrap_or_default()}
            </button>
        }
    });

    let multi_click = option.multi_select.clone().map(|multi_click| {
        let option_node = multi_click
            .iter()
            .map(|_| NodeRef::default())
            .collect::<Vec<_>>();

        let option_node = option_node.clone();
        let select = multi_click
            .clone()
            .iter()
            .cloned()
            .enumerate()
            .map(move |(i, x)| {
                let on_click = x.on_change.clone();
                let (v_label, v_bool): (_, bool) = match x.msg.clone() {
                    crate::TotalMsg::TargetMsg(crate::targets::Msgs::MyLabel(msg)) => {
                        let value = msg.get_value().clone();
                        let vs = value.split(",").map(|x| x.to_string()).collect::<Vec<_>>();
                        (vs[0].clone(), vs[1].clone().parse().unwrap())
                    }
                    _ => panic!("not multi selelct  msg"),
                };
                let option_node_copy = option_node.clone();
                use_effect(move || {
                    if let Some(input) =
                        &option_node_copy.get(i).unwrap().cast::<HtmlInputElement>()
                    {
                        input.set_checked(v_bool);
                    }
                    || ()
                });
                let event_label = v_label.clone();
                let select_input_ref_copy = option_node.get(i).unwrap().clone();
                let event = move |_e| {
                    let s = select_input_ref_copy
                        .clone()
                        .cast::<HtmlInputElement>()
                        .map(|select| select.checked())
                        .unwrap_or_default();
                    on_click.emit(format!("{},{}", event_label, s))
                };
                let select_input_ref_copy_2 = option_node.get(i).unwrap().clone();
                html! {
                    <div class="h-8 rounded-full px-1.5 py-0.5  bg-indigo-500 text-indigo-100 flex">
                        <div>
                            <input type="checkbox" class=""
                                ref={select_input_ref_copy_2.clone()}
                                onclick={event}
                                />
                        </div>
                        <div class="text-white">
                            {v_label.clone()}
                        </div>
                    </div>
                }
            })
            .collect::<Vec<_>>();
        select
    });
    debug!("multi_click {:?}", multi_click.is_some());

    html! {
        <div> //key
            <div class={search_bar_classes}>
                {match select {
                    Some(select) => select,
                    None => html! {},
                }}
                {match label_text {
                    Some(label_text) => label_text,
                    None => html! {},
                }}
                {match multi_click {
                    Some(multi_click) => html!{
                        <div class="md-5 flex ">
                            {multi_click.iter().cloned().collect::<Html>()}
                        </div>
                    },
                    None => html! {},
                }}
                {match input {
                    Some(input) => input,
                    None => html! {},
                }}
                {match duration {
                    Some(duration) => duration,
                    None => html! {},
                }}
                {match click {
                    Some(click) => click,
                    None => html! {},
                }}
            </div>
        </div>
    }
}
