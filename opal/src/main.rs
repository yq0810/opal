mod app;
use app::*;
mod components;
mod strategys;
// pub mod r#type;
pub mod types;
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
    yew::start_app::<App>();
}

#[cfg(test)]
mod test_super {

    use super::*;

    #[test]
    fn test_() {
        
        
        
    }
}