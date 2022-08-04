use crate::func_components::SettingInput;
use crate::triggers::t1::T1Msg;
use crate::triggers::{self, Msgs, T2Msg, TriggerConfig};

use crate::{AsSettingOption, TotalMsgScope};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    T1OptionUpdate(T1Msg),
    T2OptionUpdate(T2Msg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<TriggerConfig>,
    pub config: TriggerConfig,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TriggerOptions {
    pub config: TriggerConfig,
}

impl Component for TriggerOptions {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            config: props.config.clone(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let strategy_inputs =
            <triggers::Msgs as AsSettingOption>::get_options::<Msg, Msgs, TriggerOptions>(
                &self.config,
                TotalMsgScope::TriggerMsgScope(ctx.link().clone()),
            );
        html! {
            <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 w-full">
                <p class="text-2xl dark:text-slate-50 text-slate-900 px-5">
                    {"Tirgger Condition option"}
                </p>
                { strategy_inputs.iter().map(|option| {
                    html!{
                        <div class="mx-2">
                            <SettingInput {option} />
                        </div>
                    }
                }).collect::<Html>()
                }
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::T1OptionUpdate(msg) => match msg {
                T1Msg::UpdatePercentage(x) => {
                    self.config.t1.percentage = x.unwrap_or_default();
                }
                T1Msg::UpdateActive(x) => {
                    self.config.t1.active = x.unwrap_or_default();
                }
            },
            Msg::T2OptionUpdate(msg) => match msg {
                T2Msg::UpdatePercentage(x) => {
                    self.config.t2.percentage = x.unwrap_or_default();
                }
                T2Msg::UpdateActive(x) => {
                    self.config.t2.active = x.unwrap_or_default();
                }
            },
        }
        ctx.props().onupdate.emit(self.config.clone());
        true
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {}
}
