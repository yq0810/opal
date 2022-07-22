use concat_string::concat_string;
use yew::{classes, html, Html};

use crate::{
    ActivePriceResult, FloorPriceResult, SQLResult, StrategyOne, StrategyTwo, TargetResult,
};

pub struct HTMLDisplay {
    pub is_s_1: bool,
    pub is_s_2: bool,
    pub fp: Option<FloorPriceResult>,
    pub one: Option<StrategyOne>,
    pub two: Option<StrategyTwo>,
    pub target: TargetResult,
    pub diff_p: i32,
}

impl HTMLDisplay {
    pub fn new(&self) -> Html {
        let card_classes = classes!(
            "dark:bg-slate-700",
            "bg-white",
            "rounded-md",
            "w-full",
            "drop-shadow-light",
            "px-4",
            "py-4",
            "dark:text-slate-50",
            "border-solid",
            "border-l-[6px]",
            "border-blue-500",
        );
        let (earn, earn_p) = {
            match (self.target.clone(), self.target.compare_ap.clone()) {
                (t, ap) => (
                    t.profit_sale_at(&ap).unwrap_or_default(),
                    t.profit_p_sale_at(&ap).unwrap_or_default(),
                ),
            }
        };
        let is_good = earn_p as i32 >= self.diff_p;
        html! {
            <div
                class={
                    let mut c = card_classes.clone();
                    if self.fp.is_some() {
                        c.push("border-green-500");
                    }else{
                        c.push("border-red-500");
                    }
                    c
                }
            >
            <div>
            {
                    html!{
                        <div class="text-center">
                            <p>
                                <span class="text-white">
                                    {self.target.display()}
                                </span>
                            </p>
                        </div>
                    }
            }
            </div>
            <div>
            {
                match self.fp.clone() {
                    Some(x) => {

                    html!{
                        <div class="text-center">
                            <p>
                                <span class="text-green-500">
                                    {x.display()}
                                </span>
                            </p>
                        </div>
                    }},
                    None =>
                    html!{
                        <div class="text-center">
                            <p>
                                <span class="text-red-500">
                                    {"No Floor Price"}
                                </span>
                            </p>
                        </div>
                    },
                }
            }
            </div>
            <div>
                <div class="text-center">
                    <p>
                        <span class="text-green-500">
                            {self.target.compare_ap.clone().display()}
                        </span>
                    </p>
                </div>
            </div>
            <div>
            {
                match self.one.clone() {
                    Some(x) => {
                    html!{
                        <div class="text-center">
                            <p>
                               {
                                if self.is_s_1 {
                                    html!{
                                        <span class="text-blue-500">
                                            {format!("{:?}",x)}
                                        </span>
                                    }

                                }else{
                                    html!{

                                        <span class="text-red-500">
                                            {format!("{:?}",x)}
                                        </span>
                                    }

                                }

                               }
                            </p>
                        </div>
                    }},
                    None =>
                    html!{
                        <div class="text-center">
                            <p>
                                <span class="text-red-500">
                                    {"No One "}
                                </span>
                            </p>
                        </div>
                    },
                }
            }
            </div>
            <div>
            {
                match self.two.clone() {
                    Some(x) => {
                    html!{
                        <div class="text-center">
                            <p>
                               {
                                if self.is_s_2 {
                                    html!{
                                        <span class="text-blue-500">
                                            {format!("{:?}",x)}
                                        </span>
                                    }

                                }else{
                                    html!{

                                        <span class="text-red-500">
                                            {format!("{:?}",x)}
                                        </span>
                                    }

                                }

                               }
                            </p>
                        </div>
                    }},
                    None =>
                    html!{
                        <div class="text-center">
                            <p>
                                <span class="text-red-500">
                                    {"No One "}
                                </span>
                            </p>
                        </div>
                    },
                }
            }
            </div>
            <div>

                <p class={if is_good {"text-green-500"}else{"text-red-500"}}>
                {concat_string!(
                    "Profit ",earn.to_string()," ETH"," (",earn_p.to_string(),"%)")}</p>
            </div>
            // {
                // let cla = match i {
                //     0 => "flex flex-col items-start justify-end gap-1",
                //     _ => "flex flex-col justify-center1"
                // };
                // let cla2 = match i {
                //     0 => "flex flex-col items-start justify-end",
                //     _ => "flex flex-col items-start justify-end gap-5",
                // };
                // html!{
                //     <div class={cla}>
                //         <p class={cla2}>{br}</p>
                //     </div>
                // }
            // }
            </div>
        }
    }
}
