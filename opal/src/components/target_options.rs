use crate::func_components::SettingInput;

use crate::targets::full::FullMsg;
use crate::targets::{self, TargetConfig};
use crate::{AsSettingOption, TotalMsgScope};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    FullOptionUpdate(FullMsg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<TargetConfig>,
    pub config: TargetConfig,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TargetOptions {
    pub config: TargetConfig,
}

impl Component for TargetOptions {
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
            <targets::Msgs as AsSettingOption>::get_options::<Msg, targets::Msgs, TargetOptions>(
                &self.config,
                TotalMsgScope::TargetMsgScope(ctx.link().clone()),
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
            Msg::FullOptionUpdate(_) => todo!(),
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
