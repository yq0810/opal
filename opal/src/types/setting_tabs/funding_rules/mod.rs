pub mod total_amount_limit;
pub mod total_tx_count_limit;
use opal_derive::{AsSettingOptionMacro, CallbackMsgMacro, SettingCallbackFnMacro, ValueOPMacro};
pub use unit_price_limit::*;

pub mod unit_price_limit;
use self::total_amount_limit::{TotalAmountLimit, TotalAmountLimitMsg};
use self::total_tx_count_limit::{TotalTxCountLimit, TotalTxCountLimitMsg};
use crate::components;
use crate::traits::DebugConfig;
use crate::SettingCallbackFn;
pub use unit_price_limit::*;

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("funding_rule_options")]
pub struct FundingRuleConfig {
    pub total_amount_limit: TotalAmountLimit,
    pub total_tx_count_limit: TotalTxCountLimit,
    pub unit_price_limit: UnitPriceLimit,
}

impl DebugConfig for FundingRuleConfig {}

#[derive(Clone, Debug, PartialEq, ValueOPMacro, SettingCallbackFnMacro, CallbackMsgMacro)]
#[page("funding_rule_options")]
pub enum Msgs {
    TotalAmountLimit(TotalAmountLimitMsg),
    TotalTxCountLimit(TotalTxCountLimitMsg),
    UnitPriceLimit(UnitPriceLimitMsg),
}
