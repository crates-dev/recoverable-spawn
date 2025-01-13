use std::{future::Future, sync::Arc};

/// Trait alias for asynchronous functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must return a `Future` and satisfy
///   `FnMut() -> Future + Send + Sync + 'static`.
pub trait AsyncRecoverableFunction: Send + Sync + 'static {
    type Output: Send;
    type Future: Future<Output = Self::Output> + Send;

    /// Executes the asynchronous function.
    fn call(&mut self) -> Self::Future;
}

impl<F, Fut, O> AsyncRecoverableFunction for F
where
    F: FnMut() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = O> + Send + 'static,
    O: Send + 'static,
{
    type Output = O;
    type Future = Fut;

    fn call(&mut self) -> Self::Future {
        self()
    }
}

/// Trait alias for asynchronous error-handling functions used in a recoverable context.
///
/// - Functions implementing this trait must accept a `Arc<String>` as an error message,
///   return a `Future`, and satisfy `FnMut(Arc<String>) -> Future + Send + Sync + 'static`.
pub trait AsyncErrorHandlerFunction: Send + Sync + 'static {
    type Future: Future<Output = ()> + Send;

    /// Handles an error asynchronously.
    ///
    /// - `error`: The error message to handle.
    fn call(&mut self, error: Arc<String>) -> Self::Future;
}

impl<F, Fut> AsyncErrorHandlerFunction for F
where
    F: FnMut(Arc<String>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    type Future = Fut;

    fn call(&mut self, error: Arc<String>) -> Self::Future {
        self(error)
    }
}

/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `FnMut() + Send + Sync + 'static`.
pub trait RecoverableFunction: FnMut() + Send + Sync + 'static {}

impl<T> RecoverableFunction for T where T: FnMut() + Send + Sync + 'static {}

/// Trait alias for error-handling functions used in a recoverable context.
///
/// - Functions implementing this trait must accept a `&str` as an error message
///   and satisfy `Fn(&str) + Send + Sync + 'static`.
pub trait ErrorHandlerFunction: Fn(&str) + Send + Sync + 'static {}

impl<T> ErrorHandlerFunction for T where T: Fn(&str) + Send + Sync + 'static {}

/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `FnOnce() + Send + Sync + 'static`.
pub trait RecoverableOnceFunction: FnOnce() + Send + Sync + 'static {}

impl<T> RecoverableOnceFunction for T where T: FnOnce() + Send + Sync + 'static {}
