use gloo::timers::callback::Timeout;
use multimap::MultiMap;
use sql_js_httpvfs_rs::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::MediaQueryList;
use yew::prelude::*;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::strategys::{One, OneMsg};
use crate::types::{FloorPriceResult, Query, QueryError, SearchMode, SearchQuery, SearchResults};
use crate::{
    components::*, find_traget_from_floor_active, find_traget_from_profit, strategy_one,
    ActivePriceResult, CollResult, HTMLDisplay, SQLResult, SettingOption, TargetResult,
};

const DB_CONFIG: &str = r#"
{
    "from": "inline",
    "config": {
        "serverMode": "full",
        "requestChunkSize": 8192,
        "url": "../databases/db.sqlite3"
    }
}
"#;

const OPAL_THEME_KEY: &str = "opal_theme";
const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";

pub enum Msg {
    SearchStart(Option<i32>),
    TargetResults(Result<Vec<TargetResult>, QueryError>),
    UpdateFloor(Result<Vec<FloorPriceResult>, QueryError>),
    UpdateActive(Result<Vec<ActivePriceResult>, QueryError>),
    ShowRefresh(
        Vec<FloorPriceResult>,
        Vec<ActivePriceResult>,
        Vec<CollResult>,
        i32,
    ),
    ToggleSearchType,
    ToggleThemeMode(ThemeMode),
    OneOptionUpdate(OneMsg),
    // TimerDown,
}
impl Msg {}

#[derive(Clone, Copy, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub s_one: One,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct StrategyResult {
    pub pass_count: i32,
    pub earn: f64,
}

pub struct App {
    mode: SearchMode,
    first_load: bool,
    is_busy: bool,
    displayed_results: SearchResults,
    mql: Option<MediaQueryList>,
    pub targets: Vec<TargetResult>,
    pub floor_price: MultiMap<String, FloorPriceResult>,
    pub active_price: MultiMap<String, ActivePriceResult>,
    pub coll: MultiMap<String, CollResult>,
    pub config: Config,
    pub one_result: StrategyResult,
    timeout: Timeout,
    success_count: i32,
    earn: f64,
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

#[cfg(not(debug_assertions))]
fn timeout_handle(_:html::Scope<App>) -> Timeout {
    Timeout::new(2000, move || ())
}

#[cfg(debug_assertions)]
fn timeout_handle(link:html::Scope<App>) -> Timeout {
    Timeout::new(2000, move || link.send_message(Msg::SearchStart(Some(5))))
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
            timeout_handle(link)
        };
        let mut config = Config::default();
        config.s_one.volume_rate_value = 300;
        Self {
            mode: SearchMode::T2,
            first_load: true,
            is_busy: false,
            displayed_results: SearchResults::default(),
            mql: None,
            timeout: timeout_handle,
            targets: vec![],
            floor_price: MultiMap::new(),
            active_price: MultiMap::new(),
            success_count: 0,
            earn: 0.0,
            coll: MultiMap::new(),
            config,
            one_result: Default::default(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let callback = ctx.link().callback(|mode| Msg::ToggleThemeMode(mode));
            callback.emit(ThemeMode::Dark);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.first_load = false;
        match msg {
            Msg::TargetResults(results) => match results {
                Ok(results) => {
                    self.targets = results.clone();
                    self.is_busy = true;
                    true
                }
                Err(_) => true,
            },
            Msg::SearchStart(x) => match x {
                Some(x) => {
                    self.is_busy = true;
                    ctx.link().send_future(SearchMode::start(x));
                    true
                }
                None => true,
            },
            Msg::UpdateFloor(results) => match results {
                Ok(results) => {
                    results.iter().for_each(|x| {
                        self.floor_price.remove(&x.slug);
                        self.floor_price.insert(x.slug.clone(), x.clone());
                    });
                    true
                }
                Err(_) => true,
            },
            Msg::UpdateActive(results) => match results {
                Ok(results) => {
                    results.iter().for_each(|x| {
                        self.active_price.remove(&x.slug);
                        self.active_price.insert(x.slug.clone(), x.clone());
                    });
                    true
                }
                Err(_) => true,
            },
            Msg::ShowRefresh(f, a, c, p) => {
                self.floor_price.clear();
                self.active_price.clear();
                self.coll.clear();
                self.success_count = 0;
                self.earn = 0.0;
                self.one_result.earn = 0.0;
                self.one_result.pass_count = 0;

                f.iter().for_each(|x| {
                    self.floor_price.insert(x.slug.clone(), x.clone());
                });
                a.iter().for_each(|x| {
                    self.active_price.insert(x.slug.clone(), x.clone());
                });
                c.iter().for_each(|x| {
                    self.coll.insert(x.slug.clone(), x.clone());
                });

                // 1
                let stage_one =
                    find_traget_from_floor_active(&&self.floor_price, &self.coll, &a, p);
                match self.mode {
                    SearchMode::T1 => {
                        self.targets = stage_one;
                    }
                    SearchMode::T2 => {
                        let stage_actives = stage_one
                            .iter()
                            .map(|x| x.compare_ap.clone())
                            .collect::<Vec<_>>();
                        self.targets = find_traget_from_profit(
                            &stage_actives,
                            &&self.active_price,
                            &self.coll,
                            p,
                        );
                    }
                }
                // 2

                let shows = self
                    .targets
                    .iter()
                    .map(|x| {
                        let a = match self.floor_price.get_vec(&x.slug.slug) {
                            Some(xs) => xs
                                .iter()
                                .filter(|f| f.create_time > x.create_time)
                                .find(|f| f.price > x.price),
                            None => None,
                        }
                        .and_then(|x| Some(x.clone()));

                        let s_1 = strategy_one(
                            &x.create_time,
                            &&x.slug.slug,
                            &self.config.s_one.volume_rate_duration.to_duration(),
                            &self.config.s_one.tx_count_duration.to_duration(),
                            &self.active_price,
                        );
                        let is_s_1 = s_1.total_volume as i64 > self.config.s_one.volume_rate_value
                            && s_1.tx_count > self.config.s_one.tx_count_value;

                        let earn = x.profit_sale_at(&x.compare_ap).unwrap();
                        self.success_count += 1;
                        self.earn += &earn;
                        if is_s_1 {
                            self.one_result.earn += &earn;
                            self.one_result.pass_count += 1;
                        }
                        HTMLDisplay {
                            fp: a.clone(),
                            is_s_1: is_s_1.clone(),
                            one: Some(s_1.clone()),
                            target: x.clone(),
                            diff_p: p.clone(),
                        }
                        .new()
                    })
                    .collect::<Vec<_>>();
                self.displayed_results = shows;
                self.is_busy = false;
                true
            }
            Msg::ToggleSearchType => {
                self.mode = match &self.mode {
                    SearchMode::T1 => SearchMode::T2,
                    SearchMode::T2 => SearchMode::T1,
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
                }
            }
            Msg::OneOptionUpdate(option_inputs) => {
                match option_inputs {
                    OneMsg::UpdateVolumeRateValue(v) => {
                        self.config.s_one.volume_rate_value = v.unwrap_or_default()
                    }
                    OneMsg::UpdateVolumeRateDuration(v) => {
                        self.config.s_one.volume_rate_duration = v.unwrap_or_default()
                    }
                    OneMsg::UpdateTxCountValue(v) => {
                        self.config.s_one.tx_count_value = v.unwrap_or_default()
                    }
                    OneMsg::UpdateTxCountDuration(v) => {
                        self.config.s_one.tx_count_duration = v.unwrap_or_default()
                    }
                };
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
            "px-5",
            "my-10"
        );

        let text_ref = NodeRef::default();

        let link = ctx.link();
        let on_search = link.callback(|x: String| Msg::SearchStart(x.parse().ok()));
        let on_toggle = link.callback(|_| Msg::ToggleSearchType);
        let placeholder: &'static str = self.mode.placeholder_text();

        let option = SettingOption::new::<OneMsg>(
            |x| OneMsg::UpdateVolumeRateValue(x.parse().ok()),
            link,
            self.config.s_one.volume_rate_value.to_string().clone(),
            "VolumeRate:".to_string(),
            |x| OneMsg::UpdateVolumeRateDuration(Some(x)),
            self.config.s_one.volume_rate_duration.clone(),
        );

        let option2 = SettingOption::new::<OneMsg>(
            |x| OneMsg::UpdateTxCountValue(x.parse().ok()),
            link,
            self.config.s_one.tx_count_value.to_string().clone(),
            "TxCount:".to_string(),
            |x| OneMsg::UpdateTxCountDuration(Some(x)),
            self.config.s_one.tx_count_duration.clone(),
        );

        let options = vec![option, option2];
        let debug = format!("{:?}", self.config);
        let deubug_display = debug
            .split(",")
            .flat_map(|x| x.split("{").map(|x| x.to_string().replace("}", "")))
            .collect::<Vec<_>>();

        html! {
            <div class={root_classes}>
                <div class="flex inherit top-0 right-0 justify-end my-10">
                    <p class={title_classes.clone()}>{"NFT Simulation"}</p>
                    <div class="flex flex-col  text-white">
                        { deubug_display.iter().map(|d| {
                                html! {
                                    <div>
                                        {d}
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 ">
                        <p class="text-2xl font-title dark:text-slate-50 text-slate-900 px-5">
                            {"Strategy 1 option"}
                        </p>
                        { options.iter().map(|option| {
                            html!{
                                <div class="mx">
                                    <StrategyInput {option} first_load={self.first_load} is_busy={self.is_busy}/>
                                </div>
                            }
                        }).collect::<Html>()
                        }
                    </div>
                </div>
                <SearchBar {text_ref} {on_search} {placeholder} {on_toggle} toggle_text={self.mode.button_text()} first_load={self.first_load} is_busy={self.is_busy}/>
                if self.is_busy {
                    <SpinnerIcon />
                }
                else if !self.displayed_results.is_empty() {
                    <DisplayedResults mode_name={self.mode.button_text()} one={self.one_result.clone()} success_count={self.success_count} earn={self.earn} to_display={self.displayed_results.clone()}/>
                }
            </div>
        }
    }
}
