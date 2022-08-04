use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::area::Msgs;
use crate::components::coll_card::Msg as PMsg;
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

use crate::{AsInputType, AsTotalMsg, InputType};

#[derive(
    Clone, Debug, PartialEq, ValueOPMacro, AsTotalMsgMacro, CallbackMsgMacro, SettingCallbackFnMacro,
)]
#[totalMsgName("CollCard")]
#[page("coll_card")]
pub enum BlockMsg {
    Click(Option<BlockSetting>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlockSetting {
    pub slug: String,
    pub bool: bool,
}

impl Display for BlockSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.slug, self.bool)
    }
}

impl FromStr for BlockSetting {
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
pub struct Block {
    pub setting: BlockSetting,
    pub current: HashMap<String, BlockSetting>,
}
impl SettingList for Block {
    type T = BlockSetting;
    fn push(&self, setting: Self::T) -> Self {
        let mut current_block = self.current.clone();
        current_block.insert(setting.slug.clone(), setting.clone());

        let mut block = self.clone();
        block.current = current_block;
        block
    }

    fn remove(&self, setting: Self::T) -> Self {
        let mut current_block = self.current.clone();
        current_block.remove(&setting.slug);

        let mut block = self.clone();
        block.current = current_block;
        block
    }
}

impl SetTargetColl for BlockSetting {
    fn set_target_coll(&self, target_coll: &String) -> Self {
        let mut setting = self.clone();
        setting.slug = target_coll.clone();
        setting
    }
}

impl AsInputType for Block {
    fn input_type(&self) -> InputType {
        let exist = self.current.contains_key(&self.setting.slug);
        let setting = {
            let mut setting = self.setting.clone();
            setting.bool = !exist;
            setting
        };
        let title = if exist { "UnBlock" } else { "Block" };
        InputType::Button((
            concat_string!(title, "🚫"),
            BlockMsg::Click(Some(setting.clone())).to_total_msg(),
        ))
    }
}
