use crate::area::{self, AreaConfig, BlockMsg, FavoriteMsg, LabelMsg, Msgs};
use crate::func_components::SettingInput;
use crate::{AsSettingOption, CollInfo, FundingColl, SetTargetColl, SettingList, TotalMsgScope};
use concat_string::concat_string;
use log::debug;
use yew::{html, Callback, Component, Context, Html, Properties};

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub colls: Vec<FundingColl>,
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
        let display_active = props
            .colls
            .iter()
            .map(|x| {
                let labels_html = x
                    .labels
                    .iter()
                    .map(|x| {
                        html! {
                            <div class="md-2">{x.0.clone()}</div>
                        }
                    })
                    .collect::<Html>();
                let slug = x.db.slug.clone();
                html! {
                    <div class="px-4 flex space-x-4">
                       <div class="md-2">{slug}</div>    {labels_html}
                    </div>


                }
            })
            .collect::<Html>();
        html! {
              // main
                <div class="flex flex-col rounded-md md:w-3/4 w-11/12 min-w-0 max-w-[840px] py-4 bg-blue-200">
                   // title
                  <div class="text-xl text-center">
                      {"Information"}
                  </div>
        {
            display_active
        }
                  // 2 line


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
