use crate::area::{self, AreaConfig, BlockMsg, FavoriteMsg, LabelMsg, Msgs};
use crate::func_components::SettingInput;
use crate::traits::filter_by_coll::FilterByColl;
use crate::{
    AsSettingOption, CollInfo, FundingColl, LabelText, SetTargetColl, SettingList, TotalMsgScope, SettingCardConfig,
};
use concat_string::concat_string;
use log::debug;
use yew::{html, Callback, Component, Context, Html, Properties};

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub picked_colls: Vec<FundingColl>,
    pub except_colls: Vec<FundingColl>,
    pub active_label: Vec<LabelText>,
    pub block_colls: Vec<String>,
    pub setting_config:SettingCardConfig,
    pub area_config:AreaConfig,
}

pub struct CollPerviewCard {}

impl Component for CollPerviewCard {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        CollPerviewCard {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let display_colls = |xs: Vec<FundingColl>,is_pick:bool| {
            xs
            .iter()
            .map(|x| {
                let labels_html = x
                    .labels
                    .iter()
                    .enumerate()
                    .map(|(i, text)| {
                        let is_last = i == x.labels.len() - 1;
                        let is_label_active = props.active_label.contains(text);
                        let text_color = if is_label_active { "text-red-500" } else { "" };
                        let display = format!("Label({})",text.0.clone());
                        html! {
                            <div class={concat_string!("md-2 ",text_color)}>{display}{if !is_last {","} else {""} }</div>
                        }
                    })
                    .collect::<Html>();
                let slug = x.db.slug.clone();
                let twitter = {
                    let select = props.setting_config.target.verify_twitter.select;
                    let value = (x.db.twitter_is_verified == 1);
                    match (select,value,is_pick){
                        (true, false, false) => {
                            html!{<div class="text-red-500">{",twitter"}</div>}
                        },
                        (true, _, _) |
                        (false, _, _) => 
                            html!{<div></div>},
                    }
                };
                let opensea = {
                    let select = props.setting_config.target.verify_opensea.select;
                    let value = (x.db.is_verified == 1);
                    match (select,value,is_pick){
                        (true, false, false) => {
                            html!{<div class="text-red-500">{",opensea"}</div>}
                        },
                        (true, _, _) |
                        (false, _, _) => 
                            html!{<div></div>},
                    }
                };
                let blocked = {
                    let block = props.block_colls.contains(&x.db.slug);
                    match (block,is_pick){
                        (true, false) => {
                            html!{<div class="text-red-500">{",blocked"}</div>}
                        },
                        (true, true) |
                        (false, _) =>{
                            html!{<div></div>}
                        },
                    }
                };

                let total_volume = {
                    let total_volume = {
                        let select = props.setting_config.strategy.total_volume.volume_total_select;
                        let value = props.setting_config.strategy.total_volume.volume_total_value;
                        if select {
                            Some(value)

                        }else{
                            None
                        }
                    };
                    let fps = x.fp_last.clone().map(|x|x.total_volume);
                    if total_volume.is_some() {
                        if fps > total_volume  {
                            html!{
                                <div></div>
                            }
                        }else{
                            html!{
                                <div class="text-red-500">{format!(",total_volume({:?})",fps)}</div>
                            }
                        }                
                    }else{
                        html!{
                            <div></div>
                        }
                    }
                };
                let favorite = {
                    let select = props.setting_config.target.my_favorite.select;
                    if select && is_pick && props.area_config.favorite.filter_by_coll(x) {
                        html!{
                            <div class="text-red-500">{",favorotie"}</div>
                        }
                    }else{
                        html!{<div></div>}
                    }
                };
                let full = {
                    let select = props.setting_config.target.full.select;
                    if select && is_pick {
                        html!{
                            <div class="text-red-500">{"full,"}</div>
                        }
                    }else{
                        html!{<div></div>}
                    }
                };
                html! {
                    <div class="px-4 flex flex space-x-4">
                       <div class="text-xl">{slug}{":"}</div>
                       <div class="flex">
                            {full}
                            {labels_html}
                            {favorite}
                            {twitter}
                            {opensea}
                            {blocked}
                            {total_volume}
                       </div>
                    </div>


                }
            })
            .collect::<Html>()
        };
        html! {
              // main
                <div class="flex flex-col rounded-md md:w-3/4 w-11/12 min-w-0 max-w-[840px] py-4 bg-blue-200">
                   // title
                  <div class="text-xl text-center">
                      {"Picked Collection perview ("}{props.picked_colls.len()}{")"}
                  </div>
        {
            display_colls(props.picked_colls.clone(),true)
        }

                  <div class="text-xl text-center">
                      {"Except Collection perview ("}{props.except_colls.len()}{")"}
                  </div>
        {
            display_colls(props.except_colls.clone(),false)
        }


                </div>

        // <template>
        // </template>

          }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
