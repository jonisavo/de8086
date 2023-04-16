pub mod common;
mod descriptions;
pub use descriptions::{get_description, Description};
pub mod arithmetic;
pub mod instruction;
pub mod jumps;
pub mod mov;
pub mod push_pop;
