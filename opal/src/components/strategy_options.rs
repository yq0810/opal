use super::setting_card::SettingCard;
use crate::func_components::SettingInput;
use crate::strategys::{Msgs, One, StrategyConfig, Two};
use crate::InputsProps;
use crate::{
    pages::Config,
    strategys::{OneMsg, TwoMsg},
    SettingOption,
};
use yew::html::Scope;
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    OneOptionUpdate(OneMsg),
    TwoOptionUpdate(TwoMsg),
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
        fn option_list(link: &Scope<StrategyOptions>, config: &StrategyConfig) -> Vec<InputsProps> {
            let option = SettingOption::new(
                |x| Msgs::One(OneMsg::UpdateVolumeRateValue(x.parse().ok())),
                link,
                config.s_one.volume_rate_value.to_string().clone(),
                "VolumeRate:".to_string(),
                Some((
                    |x| Msgs::One(OneMsg::UpdateVolumeRateDuration(Some(x))),
                    config.s_one.volume_rate_duration.clone(),
                )),
            );

            let option2 = SettingOption::new(
                |x| Msgs::One(OneMsg::UpdateTxCountValue(x.parse().ok())),
                link,
                config.s_one.tx_count_value.to_string().clone(),
                "TxCount:".to_string(),
                Some((
                    |x| Msgs::One(OneMsg::UpdateTxCountDuration(Some(x))),
                    config.s_one.tx_count_duration.clone(),
                )),
            );

            let option3 = SettingOption::new(
                |x| Msgs::Two(TwoMsg::UpdateVolumeTotalValue(x.parse().ok())),
                link,
                config.s_two.volume_total_value.to_string().clone(),
                "TotalVolume:".to_string(),
                None,
            );

            let options = vec![option, option2, option3];
            options
                .iter()
                .map(|x| InputsProps::Strategy(x.clone()))
                .collect()
        }
        let strategy_inputs = option_list(ctx.link(), &self.config);

        html! {
            <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 ">
                <p class="text-2xl dark:text-slate-50 text-slate-900 px-5">
                    {"Strategy option"}
                </p>
                { strategy_inputs.iter().map(|option| {
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
        match msg {
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
                ctx.props().onupdate.emit(self.config.clone());
                true
            }
            Msg::TwoOptionUpdate(option_inputs) => {
                match option_inputs {
                    TwoMsg::UpdateVolumeTotalValue(v) => {
                        self.config.s_two.volume_total_value = v.unwrap_or_default()
                    }
                }
                ctx.props().onupdate.emit(self.config.clone());
                true
            }
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &yew::Context<Self>) {}
}
