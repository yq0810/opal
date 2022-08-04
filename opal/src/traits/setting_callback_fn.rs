pub trait SettingCallbackFn<M> {
    fn msg_fn() -> Box<dyn Fn(Self) -> M>;
}
