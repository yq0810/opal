use crate::area::{self, AreaConfig, BlockMsg, FavoriteMsg, LabelMsg, Msgs};
use crate::func_components::SettingInput;
use crate::{AsSettingOption, CollInfo, SetTargetColl, SettingList, TotalMsgScope};
use concat_string::concat_string;
use log::debug;
use yew::{html, Callback, Component, Context, Html, Properties};

pub enum Msg {
    FavoriteOptionUpdate(FavoriteMsg),
    BlockOptionUpdate(BlockMsg),
    LabelOptionUpdate(LabelMsg),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onupdate: Callback<AreaConfig>,
    pub coll: CollInfo,
    pub config: AreaConfig,
}

pub struct CollCard {
    pub config: AreaConfig,
}

impl Component for CollCard {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let config = props.config.clone();
        CollCard {
            config: config.set_target_coll(&props.coll.slug),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let coll_area_inputs = <area::Msgs as AsSettingOption>::get_options::<Msg, Msgs, CollCard>(
            &props.config,
            TotalMsgScope::CollCardMsgScope(ctx.link().clone()),
        );
        let coll = props.coll.clone();
        let (floor_price, v24h, vt) = {
            let data = props.coll.floor_price_result.clone().map(|x| x);
            let f = data
                .as_ref()
                .map(|x| x.price.to_string())
                .unwrap_or("unknow".to_string());
            let v24h = coll
                .volume_in_24h
                .map(|x| x.to_string())
                .unwrap_or("unknow".to_string());
            let vt = data
                .as_ref()
                .map(|x| x.total_volume.to_string())
                .unwrap_or("unknow".to_string());
            (f, v24h, vt)
        };
        let display_info = {
            |title: &'static str, value: String| -> Html {
                html! {
                    <div class="flex p-2">
                        <div class="flex flex-col text-center">
                            <div class="text-xl font-bold">{title}</div>
                            <div>{value}</div>
                        </div>
                    </div>
                }
            }
        };
        let label_map = self.config.label.current.clone();
        let label_display = {
            |label_text: String| -> Html {
                let onclick = {
                    let label_text = label_text.clone();
                    let callback = ctx.link().callback(move |_| {
                        Msg::LabelOptionUpdate(LabelMsg::RemoveInputLabelValue(Some(
                            label_text.clone(),
                        )))
                    });
                    move |_e| {
                        callback.emit(());
                    }
                };
                html! {
                    <span class="rounded-full px-1.5 py-0.5 text-sm bg-indigo-500 text-indigo-100">
                        <span>{label_text}</span>
                        <span {onclick} class="font-semibold cursor-pointer">{"×"}</span>
                    </span>
                }
            }
        };
        let labels = label_map
            .clone()
            .iter()
            .map(|(key, _)| (key.clone(), label_map.get_vec(key).unwrap().clone()))
            .collect::<Vec<_>>()
            .iter()
            .filter(|(_, v)| v.contains(&props.config.label.setting.slug))
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();

        html! {
              // main
                <div class="flex flex-col rounded-md md:w-3/4 w-11/12 min-w-0 max-w-[840px] py-4 bg-blue-200">
                   // title
                  <div class="text-xl text-center">
                      {"Information"}
                  </div>
                  // 2 line
                  <div class="flex w-full ">
                      <div class="flex self-end w-32 justify-center">
                          <div class=" p-2 bg-yellow-100 text-xl text-center">
                              <p class="">{concat_string!(coll.slug.clone())}</p>
                          </div>
                          <div class="text-xl text-center">{concat_string!("🔵")}</div>
                      </div>
                      <div class="flex grow  flex-col justify-center">
                          <div class="flex  antialiased p-8 justify-center">
                              //info list
                              {display_info("Floor price",concat_string!(floor_price," ETH"))}
                              {display_info("24H Volume",concat_string!(v24h," ETH"))}
                              {display_info("Total volume",concat_string!(vt," ETH"))}

                          </div>
                          <div>
                            { labels.iter().map(|x| label_display(x.clone().into())).collect::<Html>() }
                          </div>
                      </div>
                      <div class="w-32 justify-center">
                      {    coll_area_inputs.iter().map(|option| {
                              html!{
                                  <div class="mx">
                                      <SettingInput option={option.clone()} />
                                  </div>
                              }
                           }).collect::<Html>()
                      }
                      </div>
                    </div>
                </div>

        // <template>
        // </template>

          }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        self.config = props.config.clone();
        match msg {
            Msg::FavoriteOptionUpdate(msg) => match msg {
                FavoriteMsg::Click(setting) => {
                    if let Some(setting) = setting {
                        let mut config = self.config.clone();
                        if setting.bool {
                            config.favorite = config.favorite.push(setting);
                        } else {
                            config.favorite = config.favorite.remove(setting);
                        }
                        self.config = config;
                    }
                }
            },
            Msg::BlockOptionUpdate(msg) => match msg {
                BlockMsg::Click(setting) => {
                    if let Some(setting) = setting {
                        let mut config = self.config.clone();
                        if setting.bool {
                            config.block = config.block.push(setting);
                        } else {
                            config.block = config.block.remove(setting);
                        }
                        self.config = config;
                    }
                }
            },
            Msg::LabelOptionUpdate(msg) => match msg {
                LabelMsg::UpdateInputLabelValue(input) => {
                    if let Some(input) = input {
                        if input.ends_with(" ") {
                            let new_label = input
                                .split(" ")
                                .collect::<Vec<&str>>()
                                .first()
                                .map(|x| x.clone())
                                .unwrap();
                            let mut config = self.config.clone();
                            config.label.setting.input = "".to_string();
                            config.label = config.label.push(new_label.to_string());
                            self.config = config;
                        } else {
                            let mut config = self.config.clone();
                            config.label.setting.input = input;
                            self.config = config;
                        }
                    }
                }
                LabelMsg::RemoveInputLabelValue(slug) => {
                    if let Some(slug) = slug {
                        debug!("remove label {}", slug);
                        let mut config = self.config.clone();
                        config.label = config.label.remove(slug);
                        self.config = config;
                    }
                }
            },
        };
        ctx.props().onupdate.emit(self.config.clone());
        true
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
