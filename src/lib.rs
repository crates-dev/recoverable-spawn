//! recoverable-spawn
//!
//! A thread that supports automatic recovery from panics,
//! allowing threads to restart after a panic. Useful for resilient
//! and fault-tolerant concurrency in network and web programming.

pub(crate) mod r#async;
pub(crate) mod common;
pub(crate) mod sync;

pub(crate) use std::{any::Any, panic::set_hook, sync::Arc};

pub(crate) use tokio::task::JoinError;

pub use {r#async::*, common::*, sync::*};
