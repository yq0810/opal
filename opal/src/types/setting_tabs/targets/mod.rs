use crate::{
    components,
    traits::{filter_by_coll::FilterByColl, DebugConfig},
};
use opal_derive::{AsSettingOptionMacro, OptionMsg};
pub mod full;
use crate::SettingCallbackFn;
pub use full::*;

pub mod verify_opensea;
pub use verify_opensea::*;

pub mod verify_twitter;
pub use verify_twitter::*;

pub mod my_favorite;
pub use my_favorite::*;

pub mod my_label;
pub use my_label::*;

#[derive(Clone, Debug, PartialEq, OptionMsg)]
#[page("target_options")]
pub enum Msgs {
    Full(FullMsg),
    VerifyOpensea(VerifyOpenseaMsg),
    VerifyTwitter(VerifyTwitterMsg),
    MyFavorite(MyFavoriteMsg),
    MyLabel(MyLabelMsg),
}

#[derive(Clone, Debug, Default, PartialEq, AsSettingOptionMacro)]
#[page("target_options")]
pub struct TargetConfig {
    pub full: Full,
    pub verify_opensea: VerifyOpensea,
    pub verify_twitter: VerifyTwitter,
    pub my_favorite: MyFavorite,
    pub my_label: MyLabel,
}

impl DebugConfig for TargetConfig {}
