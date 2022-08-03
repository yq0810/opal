pub trait SettingList {
    type T;
    fn push_setting(&self, setting: Self::T) -> Self;
    fn remove_setting(&self, setting: Self::T) -> Self;
}
