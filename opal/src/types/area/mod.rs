pub mod favorite;
pub use favorite::*;

pub mod block;
pub use block::*;

pub mod label;
use crate::traits::as_input_type::AsInputType;
use crate::traits::SetTargetColl;
use crate::AsSettingOption;
use crate::CallbackMsg;
use crate::SettingCallbackFn;
use crate::SettingOption;
use crate::TotalMsgScope;
use crate::ValueOP;
pub use label::*;
use opal_derive::{AsSettingOptionMacro, CallbackMsgMacro, SettingCallbackFnMacro, ValueOPMacro};
use yew::html::Scope;
use yew::{Callback, Component};

// use crate::{
//     AsInputType, AsSettingOption, CallbackMsg, SetTargetColl, SettingCallbackFn, SettingList,
//     SettingOption, TotalMsgScope, ValueOP,
// };

use crate::components::coll_card::{self, Msg as PMsg};
#[derive(Clone, Debug, PartialEq, ValueOPMacro, SettingCallbackFnMacro, CallbackMsgMacro)]
#[page("coll_card")]
pub enum Msgs {
    Favorite(FavoriteMsg),
    Block(BlockMsg),
    Label(LabelMsg),
}

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("coll_card")]
pub struct AreaConfig {
    pub favorite: Favorite,
    pub block: Block,
    pub label: Label,
}

impl SetTargetColl for AreaConfig {
    fn set_target_coll(&self, target_coll: &String) -> Self {
        let mut config = self.clone();
        config.favorite.setting = self.favorite.setting.set_target_coll(&target_coll);
        config.block.setting = self.block.setting.set_target_coll(&target_coll);
        config.label.setting = self.label.setting.set_target_coll(&target_coll);
        config
    }
}
