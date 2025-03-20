pub(crate) mod thread;

pub use std::thread::JoinHandle;
pub use thread::{r#async, sync, r#trait::*, r#type::*};
