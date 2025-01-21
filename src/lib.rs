pub(crate) mod thread;

pub use std::thread::JoinHandle;
pub use thread::{r#async, r#trait::*, r#type::*, sync};
