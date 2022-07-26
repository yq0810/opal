use crate::TotalMsg;

pub trait GetValue {
    fn get_value(&self) -> String;
    fn to_total_msg(&self) -> TotalMsg;
}
