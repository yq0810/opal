use yew::{html, Html};

pub trait DebugConfig
where
    Self: std::fmt::Debug,
{
    fn debug_display(&self) -> Html {
        let debug = format!("{:?}", self);
        let deubug_display = debug
            .split(",")
            .flat_map(|x| x.split("{").map(|x| x.to_string().replace("}", "")))
            .collect::<Vec<_>>();
        html! {
            <div class="flex inherit top-0 right-0 justify-end my-10">
                <div class="flex flex-col  text-white">
                    { deubug_display.iter().map(|d| {
                            html! {
                                <div>
                                    {d}
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        }
    }
}
