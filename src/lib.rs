//! recoverable-spawn
//!
//! A thread that supports automatic recovery from panics,
//! allowing threads to restart after a panic. Useful for resilient
//! and fault-tolerant concurrency in network and web programming.

pub(crate) mod thread;

pub(crate) use std::panic::set_hook;
pub(crate) use std::sync::Arc;

pub use thread::{r#async, sync, r#trait::*, r#type::*};
