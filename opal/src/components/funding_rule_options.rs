use crate::func_components::SettingInput;
use crate::funding_rules::total_amount_limit::TotalAmountLimitMsg;
use crate::funding_rules::total_tx_count_limit::TotalTxCountLimitMsg;
use crate::funding_rules::{FundingRuleConfig, Msgs, UnitPriceLimitMsg};

use crate::traits::debug_config::DebugConfig;
use crate::{AsSettingOption, TotalMsgScope};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    TotalAmountLimitOptionUpdate(TotalAmountLimitMsg),
    TotalTxCountLimitOptionUpdate(TotalTxCountLimitMsg),
    UnitPriceLimitOptionUpdate(UnitPriceLimitMsg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<FundingRuleConfig>,
    pub config: FundingRuleConfig,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FundingRuleOptions {
    pub config: FundingRuleConfig,
}

impl Component for FundingRuleOptions {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            config: props.config.clone(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let strategy_inputs = <Msgs as AsSettingOption>::get_options::<
            Msg,
            crate::funding_rules::Msgs,
            FundingRuleOptions,
        >(
            &self.config,
            TotalMsgScope::FundingRuleMsgScope(ctx.link().clone()),
        );
        html! {
            <div>
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
                <div>
                    {self.config.debug_display()}
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TotalAmountLimitOptionUpdate(msgs) => {
                match msgs {
                    TotalAmountLimitMsg::UpdateValue(x) => {
                        self.config.total_amount_limit.value = x.unwrap_or_default()
                    }
                    TotalAmountLimitMsg::UpdateActive(x) => {
                        self.config.total_amount_limit.active = x.unwrap_or_default()
                    }
                };
            }
            Msg::TotalTxCountLimitOptionUpdate(msgs) => match msgs {
                TotalTxCountLimitMsg::UpdateValue(x) => {
                    self.config.total_tx_count_limit.value = x.unwrap_or_default()
                }
                TotalTxCountLimitMsg::UpdateActive(x) => {
                    self.config.total_tx_count_limit.active = x.unwrap_or_default()
                }
            },

            Msg::UnitPriceLimitOptionUpdate(msgs) => match msgs {
                UnitPriceLimitMsg::UpdateActive(x) => {
                    self.config.unit_price_limit.active = x.unwrap_or_default()
                }
                UnitPriceLimitMsg::UpdateValue(x) => {
                    self.config.unit_price_limit.value = x.unwrap_or_default()
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
