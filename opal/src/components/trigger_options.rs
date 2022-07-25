use super::setting_card::SettingCard;
use crate::func_components::SettingInput;
use crate::triggers::t1::{T1Msg, T1};
use crate::triggers::{Msgs, T2Msg, TriggerConfig, T2};
use crate::InputsProps;
use crate::{pages::Config, SettingOption};
use yew::html::Scope;
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

#[derive(Clone, PartialEq)]
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
        fn option_list(link: &Scope<TriggerOptions>, config: &TriggerConfig) -> Vec<InputsProps> {
            let option2 = InputsProps::Trigger(SettingOption::new(
                |x| Msgs::T1(T1Msg::UpdatePercentage(x.parse().ok())),
                link,
                config.t1.percentage.to_string().clone(),
                "T1 FloorPrice %:".to_string(),
                None,
            ));
            let option3 = InputsProps::Trigger(SettingOption::new(
                |x| Msgs::T1(T1Msg::UpdatePercentage(x.parse().ok())),
                link,
                config.t1.percentage.to_string().clone(),
                "T2 Profit %:".to_string(),
                None,
            ));

            let options = vec![option2, option3];
            options
        }
        let strategy_inputs = option_list(ctx.link(), &self.config);
        html! {
            <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 ">
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
            Msg::T1OptionUpdate(_) => todo!(),
            Msg::T2OptionUpdate(_) => todo!(),
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}
}
