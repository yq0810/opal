use crate::{func_components::*, SettingDuration, SettingOption};
use web_sys::{HtmlInputElement, KeyboardEvent, Node};
use yew::{classes, function_component, functional::*, html, Callback, NodeRef, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct StrategyInputProps {
    // pub label_text:String,
    // pub node_ref: NodeRef,
    // pub on_change: Box<Callback<String>>,
    pub option: SettingOption,
    pub first_load: bool,
    pub is_busy: bool,
}

#[function_component(StrategyInput)]
pub fn strategy_input(props: &StrategyInputProps) -> Html {
    let search_bar_wrap_classes = classes!("md:w-3/4", "w-11/12", "min-w-0", "max-w-[840px]",);

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
    let input_classes = classes!(
        "dark:bg-slate-700",
        "bg-white",
        "placeholder:text-gray-400",
        "placeholder:dark:text-gray-500",
        "dark:text-slate-50",
        "font-body",
        "text-base",
        "focus:outline-none",
        "flex-1",
        "min-w-0",
        "rounded-md",
        "border-blue-500"
    );
    let input_classes = "focus:outline-none rounded-none rounded bg-gray-50 border border-gray-300 text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 w-12 text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
    let input_classes_duration = "focus:outline-none rounded-none rounded-r-lg bg-gray-50 border border-gray-300 text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
    let toggle_classes = classes!(
        "h-10",
        "w-30",
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
    let x_mark_classes = classes!("w-4", "h-4", "text-slate-400", "hover:text-slate-500");
    let icon_classes = classes!("w-5", "h-5");

    let text_empty = use_state_eq(|| true);
    let input_ref = NodeRef::default();
    let input_duration_value_ref = NodeRef::default();
    if props.first_load {
        {
            let input_ref = input_ref.clone();
            let option = props.option.clone();
            let input_ref = input_ref.clone();
            let data_ref = option.input.data_ref;
            use_effect(move || {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    // let _ = input.focus();
                    input.set_value(&data_ref);
                }
                || ()
            });
            let input_duration_value_ref = input_duration_value_ref.clone();
            let duration_ref_o = option.duration.and_then(|x| Some(x.data_ref.clone()));
            match duration_ref_o {
                Some(x) => {
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
        }
    }

    let oninput_duration_value = {
        let input_ref = input_duration_value_ref.clone();
        let duration_o = props.option.clone().duration;
        let duration = match duration_o {
            Some(duration) => Some(move |_e| {
                let s = input_ref
                    .cast::<HtmlInputElement>()
                    .map(|input| input.value())
                    .unwrap_or_default();
                let nx = s.parse::<i32>().unwrap_or_default();
                let back = duration.data_ref.set_value(nx);
                duration.on_change.emit(back)
            }),
            None => None,
        };
        duration
    };

    let oninput = {
        let text_empty = text_empty.clone();
        let input_ref = input_ref.clone();
        let on_search = props.option.input.on_change.clone();
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
        let duration_o = props.option.clone().duration;
        match duration_o {
            Some(duration) => Some(move |_e| {
                let back = match duration.data_ref {
                    crate::SettingDuration::Days(x) => SettingDuration::Hours(x),
                    crate::SettingDuration::Hours(x) => SettingDuration::Days(x),
                };
                duration.on_change.emit(back)
            }),
            None => None,
        }
    };
    let (is_d, time_duration) = match props.option.duration.clone() {
        Some(d) => (true, d.data_ref.Display()),
        None => (false, "".to_string()),
    };

    html! {
        <div class={search_bar_wrap_classes}>
            <div class={search_bar_classes}>
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
