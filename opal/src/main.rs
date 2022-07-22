pub mod components;
mod func_components;
mod strategys;
mod triggers;
// pub mod r#type;
mod pages;
pub mod types;
use pages::Index;
pub use types::*;

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

#[cfg(test)]
mod test_super {

    use super::*;

    #[test]
    fn test_() {}
}
