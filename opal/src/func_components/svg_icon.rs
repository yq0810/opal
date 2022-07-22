use concat_string::concat_string;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
pub enum SvgIcons {
    Home,
    Bookmarks,
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
        let head = |x: Html| {
            html! {
                <svg
              class={concat_string!(text_color," h-6 w-6 transition-all ease-out transition-medium")}
              viewBox="0 0 24 24"
              fill="currentColor"
            >
            {x}
            </svg>
            }
        };
        let path = match &self {
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
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M4 5a3 3 0 013-3h10a3 3 0 013 3v16a1 1 0 01-1.447.894L12 18.618l-6.553 3.276A1 1 0 014 21V5zm3-1a1 1 0 00-1 1v14.382l5.553-2.776a1 1 0 01.894 0L18 19.382V5a1 1 0 00-1-1H7z"
                />

                  }
            }
        };
        head(path)
    }
}

#[function_component(SvgIcon)]
pub fn svg_icon(props: &SvgIconProps) -> Html {
    html! {
      {props.icon.to_html(props.is_active)}
    }
}
