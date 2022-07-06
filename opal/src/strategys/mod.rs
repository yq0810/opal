pub mod volume;
pub use volume::*;

pub mod one;
pub use one::*;

use crate::app::Msg;

pub trait Strategy {
    fn msgFn() -> Box<dyn Fn(Self) -> Msg>;
}