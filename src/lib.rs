pub(crate) mod thread;

pub use std::thread::JoinHandle;
pub use thread::{r#trait::*, r#type::*, spawn::*};
