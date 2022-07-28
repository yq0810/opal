pub trait SettingCallbackFn<M> {
    fn msgFn() -> Box<dyn Fn(Self) -> M>;
}
