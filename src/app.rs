use std::collections::HashMap;

use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};

use concat_string::concat_string;
use indexmap::IndexSet;
use js_sys::Function;
use serde::Deserialize;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaQueryList;
use yew::prelude::*;
use chrono::NaiveDateTime;


#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::components::*;
use crate::r#type::{SearchResults, SearchMode, SearchQuery, Query, FloorResult};

const DB_CONFIG: &str = r#"
{
    "from": "inline",
    "config": {
        "serverMode": "full",
        "requestChunkSize": 1024,
        "url": "../databases/db.sqlite3"
    }
}
"#;

const OPAL_THEME_KEY: &str = "opal_theme";
const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";

#[derive(Clone)]
pub enum Msg {
    SearchStart(SearchQuery),
    Results(SearchResults),
    ToggleSearchType,
    ToggleThemeMode(ThemeMode),
    CycleThemeMode,
    // TimerDown,
}

#[derive(Clone, Copy, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

pub struct App {
    mode: SearchMode,
    first_load: bool,
    is_busy: bool,
    displayed_results: SearchResults,
    current_theme_mode: ThemeMode,
    mql: Option<MediaQueryList>,
    timeout: Option<Timeout>,
}

// From https://github.com/yewstack/yew/issues/364#issuecomment-737138847
async fn wrap<F: std::future::Future>(f: F, finished_callback: yew::Callback<F::Output>) {
    finished_callback.emit(f.await);
}


unsafe fn initialize_worker_if_missing() {
    if !is_worker_initialized() {
        // This is *really* dumb but I don't think JsValue can just parse from
        // a string -> object.
        let v: serde_json::Value = serde_json::from_str(DB_CONFIG).unwrap();
        let x = JsValue::from_serde(&v).unwrap();
        spawn_local(async {
            create_db_worker(vec![x], "./static/code/sqlite.worker.js", "./sql-wasm.wasm").await;
        });
        // TODO: handle failure properly with some message.
    }
}

fn theme_mode() -> ThemeMode {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(res)) = local_storage.get_item(OPAL_THEME_KEY) {
                if res == DARK_THEME {
                    return ThemeMode::Dark;
                } else if res == LIGHT_THEME {
                    return ThemeMode::Light;
                }
            }
        }
    }

    ThemeMode::System
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        unsafe {
            initialize_worker_if_missing();
        }

        let timeout_handle = {
            let link = _ctx.link().clone();
            Timeout::new(1000, move || link.send_message(Msg::SearchStart(SearchQuery::FloorPrcieWithSlug("azuki".to_string(),None))))
        };
        Self {
            mode: SearchMode::Normal,
            first_load: true,
            is_busy: false,
            displayed_results: SearchResults::default(),
            current_theme_mode: theme_mode(),
            mql: None,
            timeout: Some(timeout_handle),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let callback = ctx.link().callback(|mode| Msg::ToggleThemeMode(mode));
            callback.emit(ThemeMode::Dark);

            // let callback2 = ctx.link().callback(|s| Msg::SearchStart(s));
            // callback2.emit("azuki".to_string());
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.first_load = false;
        match msg {
            Msg::SearchStart(search) => {
                // initialize_worker_if_missing();
                self.is_busy = true;
                // let query_type = 
                spawn_local(wrap(
                    SearchQuery::query(search),
                    ctx.link().callback(|results| Msg::Results(results)),
                ));
                true
            }
            Msg::Results(results) => {
                self.displayed_results = results;
                self.is_busy = false;
                true
            }
            Msg::ToggleSearchType => {
                self.mode = match &self.mode {
                    SearchMode::Normal => SearchMode::Normal,
                };
                true
            }
            Msg::ToggleThemeMode(mode) => {
                fn toggle_dark(enable: bool) {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(document_element) = document.document_element() {
                                if enable {
                                    let _ = document_element.class_list().add_1("dark");
                                } else {
                                    let _ = document_element.class_list().remove_1("dark");
                                }
                            }
                        }
                    }
                }

                match mode {
                    ThemeMode::Dark => {
                        if let Some(window) = web_sys::window() {
                            if let Some(mql) = &mut self.mql {
                                mql.set_onchange(None);
                            }

                            if let Ok(Some(local_storage)) = window.local_storage() {
                                let _ = local_storage.set_item(OPAL_THEME_KEY, DARK_THEME);
                            }
                        }
                        toggle_dark(true);

                        true
                    }
                    ThemeMode::Light => {
                        if let Some(window) = web_sys::window() {
                            if let Some(mql) = &mut self.mql {
                                mql.set_onchange(None);
                            }

                            if let Ok(Some(local_storage)) = window.local_storage() {
                                let _ = local_storage.set_item(OPAL_THEME_KEY, LIGHT_THEME);
                            }
                        }
                        toggle_dark(false);

                        true
                    }
                    ThemeMode::System => {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(mql)) =
                                window.match_media("(prefers-color-scheme: dark)")
                            {
                                toggle_dark(mql.matches());

                                // TODO: Use a closure to properly hook into Yew state.
                                // Maybe see https://github.com/rustwasm/wasm-bindgen/issues/843 and
                                // https://stackoverflow.com/a/19014495
                                mql.set_onchange(Some(&Function::new_with_args(
                                    "e",
                                    "
                                if (e.matches) {
                                    document.documentElement.classList.add('dark')
                                } else {
                                    document.documentElement.classList.remove('dark')
                                }
                                ",
                                )));

                                self.mql = Some(mql);
                            }

                            if let Ok(Some(local_storage)) = window.local_storage() {
                                let _ = local_storage.remove_item(OPAL_THEME_KEY);
                            }

                            true
                        } else {
                            false
                        }
                    }
                }
            }
            Msg::CycleThemeMode => {
                self.current_theme_mode = match self.current_theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::System,
                    ThemeMode::System => ThemeMode::Dark,
                };

                let callback = ctx.link().callback(|mode| Msg::ToggleThemeMode(mode));
                callback.emit(self.current_theme_mode);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let root_classes = classes!(
            "h-screen",
            "flex",
            "flex-col",
            "items-center",
            "justify-start",
            "gap-4",
            "dark:bg-slate-800",
            "bg-slate-100",
            "overflow-y-auto",
        );
        let title_classes = classes!(
            "text-6xl",
            "pt-16",
            "pb-6",
            "font-title",
            "dark:text-slate-50",
            "text-slate-900",
        );
        let option_div_classes = classes!(
            "absolute",
            "top-0",
            "right-0",
            "mr-[20px]",
            "mt-[18px]",
            "flex",
            "flex-row",
            "gap-x-4",
        );
        let option_button_classes = classes!(
            "flex",
            "items-center",
            "justify-center",
            "p-1.5",
            "hover:bg-slate-300",
            "hover:dark:bg-slate-600",
            "rounded-md"
        );
        let mode_button_div_classes = classes!("h-5", "w-5", "text-blue-400");
        let modal_back_classes = classes!(
            "hidden",
            "fixed",
            "top-0",
            "left-0",
            "w-full",
            "h-full",
            "outline-none",
            "overflow-x-hidden",
            "overflow-y-auto",
            "flex",
            "justify-center",
            "z-50",
            "bg-gray-700",
            "bg-opacity-50",
        );
        let modal_classes = classes!(
            "relative",
            "rounded-md",
            "overflow-hidden",
            "bg-white",
            "dark:bg-slate-700",
            "text-left",
            "w-full",
            "max-w-2xl",
            "h-full",
            "md:h-auto",
            "m-auto",
            "p-8",
            "flex",
            "flex-col",
            "gap-5",
            "drop-shadow-light",
        );
        let modal_header = classes!(
            "text-lg",
            "font-body",
            "font-bold",
            "dark:text-slate-50",
            "subpixel-antialiased",
            "pb-0.5",
        );
        let modal_text = classes!(
            "md:leading-snug",
            "leading-snug",
            "font-body",
            "text-base",
            "dark:text-slate-50",
            "subpixel-antialiased",
        );
        let link_hover = classes!(
            "text-blue-500",
            "dark:text-blue-400",
            "hover:text-blue-700",
            "hover:dark:text-blue-300"
        );

        let text_ref = NodeRef::default();
        let link = ctx.link();
        let on_search = link.callback(|s: String| {
            Msg::SearchStart(SearchQuery::FloorPrcieWithSlug(s, None))
        });
        let on_toggle = link.callback(|_| Msg::ToggleSearchType);
        let placeholder: &'static str = self.mode.placeholder_text();
        // let open_theme_window = link.callback(|_| Msg::CycleThemeMode);
        // let open_modal = Callback::from(|_| {
        //     if let Some(window) = web_sys::window() {
        //         if let Some(document) = window.document() {
        //             if let Some(modal) = document.get_element_by_id("infoModal") {
        //                 let _ = modal.class_list().remove_1("hidden");
        //             }
        //         }
        //     }
        // });
        let close_modal = Callback::from(|_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(modal) = document.get_element_by_id("infoModal") {
                        let _ = modal.class_list().add_1("hidden");
                    }
                }
            }
        });

        let about_text = "opal is a simple static webapp to look up the IPA phonetics of English words, or vice versa. More language support or sources may be added in the future.";

        html! {
            <div class={root_classes}>
                <div class={option_div_classes}>
                    // <button title="Change theme" class={option_button_classes.clone()} onclick={open_theme_window}>
                    //     <div class={mode_button_div_classes.clone()}>
                    //     {
                    //         match self.current_theme_mode {
                    //             ThemeMode::Dark => html!{<MoonIcon />},
                    //             ThemeMode::Light => html!{<SunIcon />},
                    //             ThemeMode::System => html!{<ComputerIcon />},
                    //         }
                    //     }
                    //     </div>
                    // </button>
                    // <button class={option_button_classes} onclick={open_modal}>
                    //     <div class={mode_button_div_classes}>
                    //         <InfoIcon/>
                    //     </div>
                    // </button>
                </div>
                <div id="infoModal" tabindex="-1" aria-hidden="true" role="dialog" aria-modal="true" class={modal_back_classes} onclick={close_modal.clone()}>
                    <div class={modal_classes} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <div class={classes!("absolute", "top-0", "right-0", "mr-[8px]", "mt-[8px]",)}>
                            <button class={classes!( "flex", "items-center", "justify-center", "rounded-md")} onclick={close_modal}>
                                <div class={classes!("w-4", "h-4", "text-slate-400", "hover:text-slate-500")}>
                                    <XMarkIcon />
                                </div>
                            </button>
                        </div>
                        <div>
                            <h1 class={modal_header.clone()}>
                                {"About"}
                            </h1>
                            <div class={modal_text.clone()}>
                                <p>
                                    {about_text}
                                </p>
                                <br/>
                                <p>
                                    {"All source code can be found "}
                                    <a href="https://github.com/ClementTsang/opal" target="_blank" class={link_hover.clone()}>{"on GitHub"}</a>
                                    {"."}
                                </p>
                            </div>
                        </div>

                        <div>
                            <h1 class={modal_header.clone()}>
                                {"Credits"}
                            </h1>
                            <p class={modal_text.clone()}>
                                {"opal would not be possible without:"}
                            </p>
                            <div class={modal_text}>
                                <ul class={classes!("list-disc", "pl-5", "pt-1")}>
                                    <li>
                                        { "English (US) IPA mappings based on " }
                                        <a href="https://github.com/cmusphinx/cmudict" target="_blank" class={link_hover.clone()}>{"CMUDict"}</a>
                                        { " (see " }
                                        <a href="https://github.com/cmusphinx/cmudict/blob/master/LICENSE" target="_blank" class={link_hover.clone()}>{"original license"}</a>
                                        {")" }
                                    </li>
                                    <li>
                                        <a href="https://phiresky.github.io/blog/2021/hosting-sqlite-databases-on-github-pages/" target="_blank" class={link_hover.clone()}>{"phiresky"}</a>
                                        { " for the idea of hosting SQLite on a static webpage, and writing libraries to do so." }
                                    </li>
                                    <li>
                                        <a href="https://yew.rs/" target="_blank" class={link_hover.clone()}>{"Yew"}</a>
                                        { ", the Rust frontend framework used to write this." }
                                    </li>
                                    <li>
                                        <a href="https://tailwindcss.com/" target="_blank" class={link_hover.clone()}>{"Tailwind CSS"}</a>
                                        { ", the CSS framework used because I'm bad at CSS." }
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
                <p class={title_classes}>{"NFT Simulation"}</p>
                <SearchBar {text_ref} {on_search} {placeholder} {on_toggle} toggle_text={self.mode.button_text()} first_load={self.first_load} is_busy={self.is_busy}/>
                if self.is_busy {
                    <SpinnerIcon />
                }
                else if !self.displayed_results.is_empty() {
                    <DisplayedResults to_display={self.displayed_results.clone()}/>
                }
            </div>
        }
    }
}
