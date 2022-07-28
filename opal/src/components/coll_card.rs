use crate::components::strategy_options::StrategyOptions;
use crate::components::trigger_options::TriggerOptions;
use crate::func_components::{NavButton, NavButtonProps, SvgIcons};
use crate::pages::Config;
use crate::strategys::{OneMsg, StrategyConfig, TwoMsg};
use crate::triggers::TriggerConfig;
use crate::{CollInfo, CollResult, SQLResult, SettingOption};
use yew::html::Scope;
use yew::{html, Callback, Component, Context, Html, Properties};

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    // pub onupdate: Callback<Config>,
    // pub current_page: Option<u32>,
    // pub first_load: bool,
    // pub is_busy: bool,
    // pub config: Config,
    pub coll: CollInfo,
}

pub struct CollCard {}

impl Component for CollCard {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        CollCard {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
              <div class="rounded-md md:w-3/4 w-11/12 min-w-0 max-w-[840px] py-4 bg-blue-200">
                <div class="flex">
                        <div class="flex  antialiased  p-8">
                            //boutton list
                            <div class="flex justify-center">
                                <div><p>{props.coll.slug_result.display()}</p></div>
                                <div><p>{props.coll.floor_price_result.clone().map(|x| x.display()).unwrap_or("".to_string())}</p></div>
                            <div/>
                        </div>
                </div>
              </div>
              </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
