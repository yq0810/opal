use crate::traits::filter_by_coll::FilterByColl;
use crate::InputSelect;
use crate::LabelText;
use opal_derive::WidgetMsg;

use crate::{AsInputType, InputType};

#[derive(Clone, Debug, PartialEq, WidgetMsg)]
#[totalMsgName("Target")]
#[page("target_options")]
pub enum MyFavoriteMsg {
    Select(Option<bool>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MyFavorite {
    pub select: bool,
}

impl AsInputType for MyFavorite {
    fn input_type(&self) -> InputType {
        InputType::Select(
            LabelText("My Favorite".to_string()),
            InputSelect(MyFavoriteMsg::Select(Some(self.select)).to_total_msg()),
        )
    }
}
