use crate::*;
use std::any::Any;

/// Type alias for a boxed dynamic type that implements `Any` and `Send`.
///
/// - Represents a boxed value that can be used for dynamic type checking (`Any`)
///   and safely transferred across threads (`Send`).
pub type BoxAnySend = Box<dyn Any + Send>;

/// Type alias for the result type returned by spawnable functions.
///
/// - `Ok(())`: Indicates successful execution of the function.
/// - `Err(BoxAnySend)`: Contains a boxed error value in case of a panic or failure.
pub type SpawnResult = Result<(), BoxAnySend>;

/// Alias for a boxed recoverable function.
///
/// - This type represents a boxed version of any function implementing the `RecoverableFunction` trait.
/// - Useful for dynamic dispatch of recoverable functions in multi-threaded contexts.
pub type BoxRecoverableFunction = Box<dyn RecoverableFunction>;

/// Alias for a boxed error handler function.
///
/// - This type represents a boxed version of any function implementing the `ErrorHandlerFunction` trait.
/// - Useful for dynamically handling errors with custom logic.
pub type BoxErrorHandlerFunction = Box<dyn ErrorHandlerFunction>;
