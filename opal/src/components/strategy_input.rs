use crate::{components::*, SettingOption, SettingDuration};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{classes, function_component, functional::*, html, Callback, NodeRef, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct StrategyInputProps {
    // pub label_text:String,
    // pub node_ref: NodeRef,
    // pub on_change: Box<Callback<String>>,
    pub option : SettingOption,
    pub first_load: bool,
    pub is_busy: bool,
}

#[function_component(StrategyInput)]
pub fn strategy_input(props: &StrategyInputProps) -> Html {
    let text_empty = use_state_eq(|| true);
    let input_ref = props.option.input.node_ref.clone();
    if props.first_load {
        {
            // let input_ref = input_ref.clone();
            // use_effect(move || {
            //     if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            //         // let _ = input.focus();
            //         // input.set_value("azuki")
            //     }
            //     || ()
            // });
        }
    }

    let search_bar_wrap_classes = classes!("md:w-3/4", "w-11/12", "min-w-0", "max-w-[840px]",);

    let search_bar_classes = classes!(
        "dark:bg-slate-700",
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
        "md:text-lg",
        "text-base",
        "h-12",
        "focus:outline-none",
        "flex-1",
        "pl-1",
        "min-w-0",
    );
    let x_button_classes = classes!(
        "dark:text-gray-400",
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "px-2"
    );
    let button_classes = classes!(
        "dark:bg-slate-700",
        "dark:text-gray-400",
        "bg-white",
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "px-4",
        "hover:bg-blue-500",
        "hover:text-slate-50",
        "hover:dark:bg-blue-500",
        "hover:dark:text-slate-50",
        "disabled:bg-white",
        "disabled:text-slate-100",
        "disabled:dark:bg-slate-700",
        "disabled:dark:text-slate-600",
    );
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
        let text_empty = text_empty.clone();
        let input_ref = input_ref.clone();
        let duration = props.option.clone().duration.unwrap();
        move |_e| {
            let back = match duration.data_ref {
                crate::SettingDuration::Days(x) => SettingDuration::Hours(x),
                crate::SettingDuration::Hours(x) => SettingDuration::Days(x),
            };
            duration.on_change.emit(back)
        }
    };
    let time_duration = props.option.duration.clone().unwrap().data_ref.Display();

    html! {
        <div class={search_bar_wrap_classes}>
            <div class={search_bar_classes}>
                <button class={toggle_classes.clone()} style="" disabled=true >{props.option.input.label_text.clone()}</button>
                <input title="Search query" class={input_classes} type="text" ref={input_ref} {oninput} />
                <button class={toggle_classes_o} onclick={toggle_onclick}>{time_duration}</button>
            </div>
        </div>
    }
}
