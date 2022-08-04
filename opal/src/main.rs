pub mod components;
mod func_components;
// pub mod r#type;
mod pages;
pub mod types;
use pages::Index;
pub use types::*;

mod traits;
use traits::*;

// pub mod r#type;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    #[cfg(feature = "console_log")]
    {
        use log::Level;
        console_log::init_with_level(Level::Trace).expect("error initializing log");
    }
    yew::start_app::<Index>();
}
