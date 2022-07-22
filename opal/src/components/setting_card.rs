use crate::components::strategy_options::StrategyOptions;
use crate::components::trigger_options::TriggerOptions;
use crate::func_components::{NavButton, NavButtonProps, SvgIcons};
use crate::pages::Config;
use crate::strategys::{OneMsg, StrategyConfig, TwoMsg};
use crate::triggers::TriggerConfig;
use crate::SettingOption;
use yew::html::Scope;
use yew::{html, Callback, Component, Context, Html, Properties};

pub enum Msg {
    ActiveTab(u32),
    StrategyConfigUpdate(StrategyConfig),
    TriggerConfigUpdate(TriggerConfig),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<Config>,
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
    let b1 = {
        let index = 0;
        NavButtonProps {
            name: "Funding Rules",
            svg: SvgIcons::Circle(1),
            index: index.clone(),
            is_active: if active_tab == index { true } else { false },
            onclick: link.callback(move |_| Msg::ActiveTab(index)),
        }
    };
    let b2 = {
        let index = 1;
        NavButtonProps {
            name: "Target Area",
            svg: SvgIcons::Circle(2),
            index: index.clone(),
            is_active: if active_tab == index { true } else { false },
            onclick: link.callback(move |_| Msg::ActiveTab(index)),
        }
    };
    let b3 = {
        let index = 2;
        NavButtonProps {
            name: "Trigger Condition",
            svg: SvgIcons::Circle(3),
            index: index.clone(),
            is_active: if active_tab == index { true } else { false },
            onclick: link.callback(move |_| Msg::ActiveTab(index)),
        }
    };
    let b4 = {
        let index = 3;
        NavButtonProps {
            name: "Execution Strategy",
            svg: SvgIcons::Circle(4),
            index: index.clone(),
            is_active: if active_tab == index { true } else { false },
            onclick: link.callback(move |_| Msg::ActiveTab(index)),
        }
    };
    vec![b1, b2, b3, b4]
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
        let select_tab = self.active_tab;
        let dynamic_css = { format!(r#"transform: translateY(calc(100% * {select_tab}))"#) };
        let nav_buttons = button_list(ctx.link(), self.active_tab);
        let strategy_config_onupdate = {
            ctx.link()
                .callback(|config| Msg::StrategyConfigUpdate(config))
        };
        let trigger_config_onupdate = {
            ctx.link()
                .callback(|config| Msg::TriggerConfigUpdate(config))
        };
        html! {
            // card board
            <div class="max-w-[840px] w-11/12 ">
              <div class="flex">
                    <div class="min-h-[440px] flex rounded-l-md antialiased bg-gray-200 p-8">
                        //boutton list
                        <div class="flex justify-center">
                            <nav id="nav" class="w-56 relative">
                                <span
                                    class="absolute h-10 w-40 bg-white rounded-lg shadow ease-out transition-transform transition-medium"
                                    style={dynamic_css}/>
                                <ul class="relative">
                                    { nav_buttons.into_iter().map(|props| {
                                        html! {
                                            <li class="relative h-10 w-50">
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
                    </div>
                    <div class="flex rounded-r-md antialiased bg-gray-400 p-8">
                        {match self.active_tab {
                            3 => html!{<StrategyOptions onupdate={strategy_config_onupdate} config={self.config.strategy.clone()}/>},
                            2 => html!{<TriggerOptions onupdate={trigger_config_onupdate} config={self.config.trigger.clone()}/>},
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
            Msg::StrategyConfigUpdate(c) => {
                self.config.strategy = c;
                ctx.props().onupdate.emit(self.config.clone());
                true
            }
            Msg::TriggerConfigUpdate(_) => todo!(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
