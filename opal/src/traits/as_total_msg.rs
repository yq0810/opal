use crate::TotalMsg;

pub trait AsTotalMsg {
    fn to_total_msg(&self) -> TotalMsg;
}
