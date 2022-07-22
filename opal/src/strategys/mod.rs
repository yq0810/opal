pub mod volume;
pub use volume::*;

pub mod one;
pub use one::*;

pub mod two;
pub use two::*;

use crate::components::setting_card::Msg;

pub trait Strategy {
    fn msgFn() -> Box<dyn Fn(Self) -> Msg>;
}
