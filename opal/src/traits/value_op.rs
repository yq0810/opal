pub trait ValueOP {
    fn get_value(&self) -> String;
    fn set_value(&self, new_value: String) -> Self;
}
