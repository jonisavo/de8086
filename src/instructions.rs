pub mod common;
mod descriptions;
pub use descriptions::{resolve, Description};
pub mod arithmetic;
pub mod data_transfer;
pub mod instruction;
pub mod jumps;
pub mod mov;
pub mod push_pop;
