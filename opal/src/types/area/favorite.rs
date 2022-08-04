use crate::area::Msgs;
use crate::ParserError;
use crate::SetTargetColl;
use crate::SettingCallbackFn;
use crate::SettingList;
use crate::TotalMsg;
use crate::ValueOP;
use concat_string::concat_string;
use opal_derive::CallbackMsgMacro;
use opal_derive::SettingCallbackFnMacro;
use opal_derive::{AsTotalMsgMacro, ValueOPMacro};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::components::coll_card::Msg as PMsg;
use crate::{AsInputType, AsTotalMsg, InputType};

#[derive(
    Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro, CallbackMsgMacro, SettingCallbackFnMacro,
)]
#[totalMsgName("CollCard")]
#[page("coll_card")]
pub enum FavoriteMsg {
    Click(Option<FavoriteSetting>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FavoriteSetting {
    pub slug: String,
    pub bool: bool,
}

impl Display for FavoriteSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.slug, self.bool)
    }
}

impl FromStr for FavoriteSetting {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_s = s.split(",").collect::<Vec<&str>>();
        Ok(Self {
            slug: str_s.get(0).unwrap().to_string(),
            bool: match str_s.get(1).unwrap() {
                &"true" => true,
                &"false" => false,
                _ => return Err(ParserError::BoolError),
            },
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Favorite {
    pub setting: FavoriteSetting,
    pub current: HashMap<String, FavoriteSetting>,
}
impl SettingList for Favorite {
    type T = FavoriteSetting;
    fn push(&self, setting: Self::T) -> Self {
        let mut current_favorite = self.current.clone();
        current_favorite.insert(setting.slug.clone(), setting.clone());

        let mut favorite = self.clone();
        favorite.current = current_favorite;
        favorite
    }

    fn remove(&self, setting: Self::T) -> Self {
        let mut current_favorite = self.current.clone();
        current_favorite.remove(&setting.slug);

        let mut favorite = self.clone();
        favorite.current = current_favorite;
        favorite
    }
}

impl SetTargetColl for FavoriteSetting {
    fn set_target_coll(&self, target_coll: &String) -> Self {
        let mut setting = self.clone();
        setting.slug = target_coll.clone();
        setting
    }
}

impl AsInputType for Favorite {
    fn input_type(&self) -> InputType {
        let exist = self.current.contains_key(&self.setting.slug);
        let setting = {
            let mut setting = self.setting.clone();
            setting.bool = !exist;
            setting
        };
        let title = if exist { "UnFavorite" } else { "Favorite" };
        InputType::Button((
            concat_string!(title, "❤️"),
            FavoriteMsg::Click(Some(setting.clone())).to_total_msg(),
        ))
    }
}
