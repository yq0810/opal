use concat_string::concat_string;
use yew::{classes, function_component, html, Html, Properties};

use crate::SearchResults;



#[derive(Clone, PartialEq, Properties)]
pub struct DisplayedResultsProps {
    pub to_display: SearchResults,
    pub success_count: i32,
    pub earn: f64,
}

#[function_component(DisplayedResults)]
pub fn displayed_results(props: &DisplayedResultsProps) -> Html {
    let results_class = classes!(
        "pb-6",
        "md:w-3/4",
        "w-11/12",
        "min-w-0",
        "max-w-[840px]",
        "flex",
        "flex-col",
        "gap-4",
    );

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

    let state_card_classes = classes!(
        "dark:bg-slate-900",
        "bg-white",
        "rounded-md",
        "w-full",
        "drop-shadow-light",
        "px-4",
        "py-4",
        "dark:text-slate-50",
        "border-solid",
        "border-l-[10px]",
        "border-blue-500",
    );
    let state_result_class = classes!(
        "flex",
        "font-body",
        "md:leading-none",
        "leading-none",
        "md:text-lg",
        "text-base",
        "dark:text-slate-50",
        "subpixel-antialiased",
    );

    let word_class = classes!(
        "font-body",
        "font-bold",
        "text-lg",
        "pb-1",
        "subpixel-antialiased"
    );

    let result_class = classes!(
        "font-body",
        "md:leading-none",
        "leading-none",
        "md:text-lg",
        "text-base",
        "dark:text-slate-50",
        "subpixel-antialiased",
    );

    html! {
        <div class={results_class}>
            <div class={state_result_class}>
                <div
                    class={
                        let mut c = state_card_classes.clone();
                        c.push("border-green-500");
                        c
                    }
                    key={concat_string!("earn","1")}
                >
                    <p>{concat_string!("target_count (alpha): ",
                                    props.to_display.len().to_string()
                                    )}
                                    </p>
                </div>
                <div
                    class={
                        let mut c = state_card_classes.clone();
                        c.push("border-green-500");
                        c
                    }
                    key={concat_string!("earn (alpha)","1")}
                >
                    <p>{concat_string!("success_count: ",
                                    props.success_count.to_string()
                                    )}
                                    </p>
                </div>
                <div
                    class={
                        let mut c = state_card_classes.clone();
                        c.push("border-green-500");
                        c
                    }
                    key={concat_string!("earn","1")}
                >
                    <p>{concat_string!("earn (alpha): ",props.earn.to_string()," ETH")}</p>
                </div>
            </div>
            <hr/>
           
            { props.to_display.iter().map(|result| {
                let is_good =  result.contains("✔");
                html!{
                    <div
                        class={
                            let mut c = card_classes.clone();
                            if is_good {
                                c.push("border-green-500");
                            }else{
                                c.push("border-red-500");
                            }
                            c
                        }
                        key={concat_string!(result,"1")}
                    >
                    { result.split("<>").collect::<Vec<_>>().iter().map(|br|{
                        html!{
                            <div>
                                <p class={word_class.clone()}>{br}</p>
                                <hr/>
                            </div>
                        }

                        }).collect::<Html>()

                    }
                    </div>
                }
            }).collect::<Html>() }
        </div>
    }
}
