use yew::{function_component, html, Callback, Properties};

use super::SvgIcons;
use crate::func_components::SvgIcon;
use concat_string::concat_string;

#[derive(Clone, PartialEq, Properties)]
pub struct NavButtonProps {
    pub index: u32,
    pub name: &'static str,
    pub svg: SvgIcons,
    pub is_active: bool,
    pub onclick: Callback<u32>,
}

#[function_component(NavButton)]
pub fn nav_button(props: &NavButtonProps) -> Html {
    let onclick = {
        let onclick_p = props.onclick.clone();
        let index_p = props.index.clone();
        move |_| onclick_p.emit(index_p)
    };
    let text_color = if props.is_active {
        "text-indigo-400"
    } else {
        "text-gray-500"
    };
    html! {
        <button type="button"
                class="py-2 px-3 w-full flex items-center focus:outline-none focus-visible:underline"
                {onclick}
                >
            <SvgIcon icon={props.svg.clone()} is_active={props.is_active}/>
            <span class={concat_string!(text_color," ml-2 text-sm font-medium transition-all ease-out transition-medium")}>
                {props.name}
            </span>
        </button>
    }
}
