use crate::func_components::SettingInput;
use crate::targets::full::FullMsg;
use crate::targets::{
    self, MyFavoriteMsg, MyLabelMsg, TargetConfig, VerifyOpenseaMsg, VerifyTwitterMsg,
};
use crate::traits::DebugConfig;
use crate::{AsSettingOption, TotalMsgScope};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    FullOptionUpdate(FullMsg),
    VerifyOpenseaOptionUpdate(VerifyOpenseaMsg),
    VerifyTwitterOptionUpdate(VerifyTwitterMsg),
    MyFavoriteOptionUpdate(MyFavoriteMsg),
    MyLabelOptionUpdate(MyLabelMsg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<TargetConfig>,
    pub config: TargetConfig,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TargetOptions {
    pub config: TargetConfig,
}

impl Component for TargetOptions {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            config: props.config.clone(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let inputs =
            <targets::Msgs as AsSettingOption>::get_options::<Msg, targets::Msgs, TargetOptions>(
                &self.config,
                TotalMsgScope::TargetMsgScope(ctx.link().clone()),
            );
        html! {
            <div>
                <div class="flex-col p-5 block p-6 max-w-sm bg-white rounded-lg border border-gray-200 shadow-md hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 w-full">
                    <p class="text-2xl dark:text-slate-50 text-slate-900 px-5">
                        {"Target option"}
                    </p>
                    { inputs.iter().map(|option| {
                        html!{
                            <div class="mx-2">
                                <SettingInput {option} />
                            </div>
                        }
                    }).collect::<Html>()
                    }
                </div>
                <div>
                    {self.config.debug_display()}
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FullOptionUpdate(msgs) => match msgs {
                FullMsg::Select(x) => self.config.full.select = x.unwrap_or_default(),
            },
            Msg::VerifyOpenseaOptionUpdate(msgs) => match msgs {
                VerifyOpenseaMsg::Select(x) => {
                    self.config.verify_opensea.select = x.unwrap_or_default()
                }
            },
            Msg::VerifyTwitterOptionUpdate(msgs) => match msgs {
                VerifyTwitterMsg::Select(x) => {
                    self.config.verify_twitter.select = x.unwrap_or_default()
                }
            },
            Msg::MyFavoriteOptionUpdate(msgs) => match msgs {
                MyFavoriteMsg::Select(x) => self.config.my_favorite.select = x.unwrap_or_default(),
            },
            Msg::MyLabelOptionUpdate(msgs) => match msgs {
                MyLabelMsg::Select(x) => self.config.my_label.select = x.unwrap_or_default(),
                MyLabelMsg::SelectMulti(x) => match x {
                    Some(my_label_setting) => {
                        self.config.my_label.selected_labels = {
                            let mut selected_labels = self.config.my_label.selected_labels.clone();
                            if my_label_setting.bool {
                                selected_labels.insert(my_label_setting.label);
                            } else {
                                selected_labels.remove(&my_label_setting.label);
                            }
                            selected_labels
                        }
                    }
                    None => (),
                },
            },
        }
        ctx.props().onupdate.emit(self.config.clone());
        true
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        self.config = ctx.props().config.clone();
        true
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {}
}
