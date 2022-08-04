use crate::func_components::SettingInput;
use crate::strategys::{self, Msgs, StrategyConfig, ThreeMsg};
use crate::strategys::{OneMsg, TwoMsg};
use crate::{AsSettingOption, TotalMsgScope};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    OneOptionUpdate(OneMsg),
    TwoOptionUpdate(TwoMsg),
    ThreeOptionUpdate(ThreeMsg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<StrategyConfig>,
    pub config: StrategyConfig,
}

#[derive(Clone, PartialEq)]
pub struct StrategyOptions {
    pub config: StrategyConfig,
}

impl Component for StrategyOptions {
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
            <strategys::Msgs as AsSettingOption>::get_options::<Msg, Msgs, StrategyOptions>(
                &self.config,
                TotalMsgScope::StrategyMsgScope(ctx.link().clone()),
            );
        html! {
            <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 w-full">
                <p class="text-2xl dark:text-slate-50 text-slate-900 px-5">
                    {"Strategy option"}
                </p>
                    {    strategy_inputs.iter().map(|option| {
                            html!{
                                <div class="mx">
                                    <SettingInput option={option.clone()} />
                                </div>
                            }
                         }).collect::<Html>()
                    }
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let t = match msg {
            Msg::OneOptionUpdate(option_inputs) => {
                match option_inputs {
                    OneMsg::UpdateVolumeRateValue(v) => {
                        self.config.s_one.volume_rate_value = v.unwrap_or_default()
                    }
                    OneMsg::UpdateVolumeRateDuration(v) => {
                        self.config.s_one.volume_rate_duration = v.unwrap_or_default()
                    }
                    OneMsg::UpdateVolumeRateSelect(v) => {
                        self.config.s_one.volume_rate_select = v.unwrap_or_default()
                    }
                };
                true
            }
            Msg::TwoOptionUpdate(option_inputs) => {
                match option_inputs {
                    TwoMsg::UpdateVolumeTotalValue(v) => {
                        self.config.s_two.volume_total_value = v.unwrap_or_default()
                    }
                    TwoMsg::UpdateVolumeTotalSelect(v) => {
                        self.config.s_two.volume_total_select = v.unwrap_or_default()
                    }
                }
                true
            }
            Msg::ThreeOptionUpdate(option_inputs) => {
                match option_inputs {
                    ThreeMsg::UpdateTxCountValue(v) => {
                        self.config.s_three.tx_count_value = v.unwrap_or_default();
                    }
                    ThreeMsg::UpdateTxCountDuration(v) => {
                        self.config.s_three.tx_count_duration = v.unwrap_or_default();
                    }
                    ThreeMsg::UpdateTxCountSelect(v) => {
                        self.config.s_three.tx_count_select = v.unwrap_or_default();
                    }
                }
                true
            }
        };
        ctx.props().onupdate.emit(self.config.clone());
        t
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {}
}
