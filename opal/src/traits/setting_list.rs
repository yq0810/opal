pub trait SettingList {
    type T;
    fn push(&self, item: Self::T) -> Self;
    fn remove(&self, item: Self::T) -> Self;
}
