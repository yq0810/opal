use crate::InputType;

pub trait AsInputType {
    fn input_type(&self) -> InputType;
}
