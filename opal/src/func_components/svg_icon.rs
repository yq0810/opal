use concat_string::concat_string;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
pub enum SvgIcons {
    Home,
    Bookmarks,
    Dollar,
    Circle(u32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SvgIconProps {
    pub icon: SvgIcons,
    pub is_active: bool,
}

impl SvgIcons {
    pub fn to_html(&self, is_active: bool) -> Html {
        let text_color = if is_active {
            "text-indigo-400"
        } else {
            "text-gray-500"
        };
        match &self {
            SvgIcons::Home => {
                html! {
                  <path
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M12 2a10 10 0 100 20 10 10 0 000-20zm0 2a8 8 0 100 16 8 8 0 000-16zm0 3a5 5 0 100 10 5 5 0 000-10zm0 4a2 2 0 100 4 2 2 0 000-4z"
                  />
                }
            }
            SvgIcons::Bookmarks => {
                html! {
                <svg
                  class={concat_string!(text_color," h-6 w-6 transition-all ease-out transition-medium")}
                  viewBox="0 0 24 24"
                  fill="currentColor"
                >
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M4 5a3 3 0 013-3h10a3 3 0 013 3v16a1 1 0 01-1.447.894L12 18.618l-6.553 3.276A1 1 0 014 21V5zm3-1a1 1 0 00-1 1v14.382l5.553-2.776a1 1 0 01.894 0L18 19.382V5a1 1 0 00-1-1H7z"
                />
                </svg>

                  }
            }
            SvgIcons::Dollar => html! {
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                  class={concat_string!(text_color," bi bi-currency-dollar")} viewBox="0 0 16 16">
                <path d="M4 10.781c.148 1.667 1.513 2.85 3.591 3.003V15h1.043v-1.216c2.27-.179 3.678-1.438 3.678-3.3 0-1.59-.947-2.51-2.956-3.028l-.722-.187V3.467c1.122.11 1.879.714 2.07 1.616h1.47c-.166-1.6-1.54-2.748-3.54-2.875V1H7.591v1.233c-1.939.23-3.27 1.472-3.27 3.156 0 1.454.966 2.483 2.661 2.917l.61.162v4.031c-1.149-.17-1.94-.8-2.131-1.718H4zm3.391-3.836c-1.043-.263-1.6-.825-1.6-1.616 0-.944.704-1.641 1.8-1.828v3.495l-.2-.05zm1.591 1.872c1.287.323 1.852.859 1.852 1.769 0 1.097-.826 1.828-2.2 1.939V8.73l.348.086z"/>
              </svg>
            },
            SvgIcons::Circle(x) => html! {
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                  class={concat_string!(text_color," bi bi-1-circle")} viewBox="0 0 16 16">
                  {match x {
                    1 => html! {
                      <path d="M1 8a7 7 0 1 0 14 0A7 7 0 0 0 1 8Zm15 0A8 8 0 1 1 0 8a8 8 0 0 1 16 0ZM9.283 4.002V12H7.971V5.338h-.065L6.072 6.656V5.385l1.899-1.383h1.312Z"/>
                    },
                    2 => html! {
                      <path d="M1 8a7 7 0 1 0 14 0A7 7 0 0 0 1 8Zm15 0A8 8 0 1 1 0 8a8 8 0 0 1 16 0ZM6.646 6.24v.07H5.375v-.064c0-1.213.879-2.402 2.637-2.402 1.582 0 2.613.949 2.613 2.215 0 1.002-.6 1.667-1.287 2.43l-.096.107-1.974 2.22v.077h3.498V12H5.422v-.832l2.97-3.293c.434-.475.903-1.008.903-1.705 0-.744-.557-1.236-1.313-1.236-.843 0-1.336.615-1.336 1.306Z"/>
                    },
                    3 => html! {
                         <>
                        <path d="M7.918 8.414h-.879V7.342h.838c.78 0 1.348-.522 1.342-1.237 0-.709-.563-1.195-1.348-1.195-.79 0-1.312.498-1.348 1.055H5.275c.036-1.137.95-2.115 2.625-2.121 1.594-.012 2.608.885 2.637 2.062.023 1.137-.885 1.776-1.482 1.875v.07c.703.07 1.71.64 1.734 1.917.024 1.459-1.277 2.396-2.93 2.396-1.705 0-2.707-.967-2.754-2.144H6.33c.059.597.68 1.06 1.541 1.066.973.006 1.6-.563 1.588-1.354-.006-.779-.621-1.318-1.541-1.318Z"/>
                        <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0ZM1 8a7 7 0 1 0 14 0A7 7 0 0 0 1 8Z"/>
                        </>
                    },
                    4 => html! {
                         <>
                        <path d="M7.519 5.057c.22-.352.439-.703.657-1.055h1.933v5.332h1.008v1.107H10.11V12H8.85v-1.559H4.978V9.322c.77-1.427 1.656-2.847 2.542-4.265ZM6.225 9.281v.053H8.85V5.063h-.065c-.867 1.33-1.787 2.806-2.56 4.218Z"/>
                        <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0ZM1 8a7 7 0 1 0 14 0A7 7 0 0 0 1 8Z"/>
                        </>
                    },
                    _ => todo!(),

                  }}
            </svg>

            },
        }
    }
}

#[function_component(SvgIcon)]
pub fn svg_icon(props: &SvgIconProps) -> Html {
    html! {
      {props.icon.to_html(props.is_active)}
    }
}
