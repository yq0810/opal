use crate::func_components::{NavButton, NavButtonProps, StrategyInput, SvgIcons};
use crate::pages::Config;
use crate::strategys::{OneMsg, TwoMsg};
use crate::SettingOption;
use yew::html::Scope;
use yew::{html, Component, Context, Html, Properties};

pub enum Msg {
    ActiveTab(u32),
    OneOptionUpdate(OneMsg),
    TwoOptionUpdate(TwoMsg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    // pub onupdate: Callback<u32>,
    // pub current_page: Option<u32>,
    pub first_load: bool,
    pub is_busy: bool,
    pub config: Config,
}

pub struct SettingCard {
    active_tab: u32,
    config: Config,
}

fn button_list(link: &Scope<SettingCard>, active_tab: u32) -> Vec<NavButtonProps> {
    let home = {
        let index = 0;
        NavButtonProps {
            name: "Home",
            svg: SvgIcons::Home,
            index: index.clone(),
            is_active: if active_tab == index { true } else { false },
            onclick: link.callback(move |_| Msg::ActiveTab(index)),
        }
    };
    let h2 = {
        let index = 1;
        NavButtonProps {
            name: "Home",
            svg: SvgIcons::Bookmarks,
            index: index.clone(),
            is_active: if active_tab == index { true } else { false },
            onclick: link.callback(move |_| Msg::ActiveTab(index)),
        }
    };
    vec![home, h2]
}

fn option_list(link: &Scope<SettingCard>, config: &Config) -> Vec<SettingOption> {
    let option = SettingOption::new::<OneMsg>(
        |x| OneMsg::UpdateVolumeRateValue(x.parse().ok()),
        link,
        config.s_one.volume_rate_value.to_string().clone(),
        "VolumeRate:".to_string(),
        Some((
            |x| OneMsg::UpdateVolumeRateDuration(Some(x)),
            config.s_one.volume_rate_duration.clone(),
        )),
    );

    let option2 = SettingOption::new::<OneMsg>(
        |x| OneMsg::UpdateTxCountValue(x.parse().ok()),
        link,
        config.s_one.tx_count_value.to_string().clone(),
        "TxCount:".to_string(),
        Some((
            |x| OneMsg::UpdateTxCountDuration(Some(x)),
            config.s_one.tx_count_duration.clone(),
        )),
    );

    let option3 = SettingOption::new::<TwoMsg>(
        |x| TwoMsg::UpdateVolumeTotalValue(x.parse().ok()),
        link,
        config.s_two.volume_total_value.to_string().clone(),
        "TotalVolume:".to_string(),
        None,
    );

    let options = vec![option, option2, option3];
    options
}

impl Component for SettingCard {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        match props.first_load {
            true => Self {
                active_tab: 0,
                config: props.config.clone(),
            },
            false => Self {
                active_tab: 0,
                config: props.config.clone(),
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        if props.first_load {
            {}
        }
        let select_tab = self.active_tab;
        let dynamic_css = { format!(r#"transform: translateY(calc(100% * {select_tab}))"#) };
        let nav_buttons = button_list(ctx.link(), self.active_tab);
        let strategy_inputs = option_list(ctx.link(), &props.config);
        html! {
            // card board
            <div class="rounded-md antialiased bg-gray-200 p-8">
                <div class="flex">
                    //boutton list
                    <div class="flex justify-center">
                        <nav id="nav" class="w-56 relative">
                            <span
                                class="absolute h-10 w-40 bg-white rounded-lg shadow ease-out transition-transform transition-medium"
                                style={dynamic_css}/>
                            <ul class="relative">
                                { nav_buttons.into_iter().map(|props| {
                                    html! {
                                        <li class="relative">
                                            <NavButton name={props.name}
                                                    svg={props.svg}
                                                    index={props.index}
                                                    is_active={props.is_active}
                                                    onclick={props.onclick}
                                                        />
                                        </li>
                                }
                                }).collect::<Html>()}
                            </ul>
                        </nav>
                    </div>
                    //tab
                    <div>
                        {match self.active_tab {
                            0 => html! {
                                <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 ">
                                    <p class="text-2xl font-title dark:text-slate-50 text-slate-900 px-5">
                                        {"Strategy option"}
                                    </p>
                                    { strategy_inputs.iter().map(|option| {
                                        html!{
                                            <div class="mx">
                                                <StrategyInput {option} first_load={props.first_load} is_busy={props.is_busy}/>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                    }
                                </div>
                            },
                            _ => html!{}
                        } }
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ActiveTab(x) => {
                self.active_tab = x;
                true
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
            Msg::TwoOptionUpdate(option_inputs) => {
                match option_inputs {
                    TwoMsg::UpdateVolumeTotalValue(v) => {
                        self.config.s_two.volume_total_value = v.unwrap_or_default()
                    }
                }
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
