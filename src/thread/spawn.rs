use super::{r#trait::*, r#type::*};
use std::panic::{self, AssertUnwindSafe};
use std::thread::{spawn, JoinHandle};

/// Executes a recoverable function within a panic-safe context.
///
/// - `func`: A function implementing the `RecoverableFunction` trait.
/// - Returns: A `SpawnResult` indicating the success or failure of the function execution.
#[inline]
pub fn run_function<F: RecoverableFunction>(func: F) -> SpawnResult {
    panic::catch_unwind(AssertUnwindSafe(|| {
        func();
    }))
}

/// Executes an error-handling function with a given error message within a panic-safe context.
///
/// - `func`: A function implementing the `ErrorHandlerFunction` trait.
/// - `error`: A string slice representing the error message.
/// - Returns: A `SpawnResult` indicating the success or failure of the error-handling function execution.
#[inline]
pub fn run_error_handle_function<E: ErrorHandlerFunction>(func: E, error: &str) -> SpawnResult {
    panic::catch_unwind(AssertUnwindSafe(|| {
        func(error);
    }))
}

/// Converts a panic-captured error value into a string.
///
/// - `err`: The captured error value, of type `BoxAnySend`.
/// - Returns: A string representation of the error value.
#[inline]
pub fn spawn_error_to_string(err: BoxAnySend) -> String {
    match err.downcast_ref::<&str>() {
        Some(str_slice) => str_slice.to_string(),
        None => match err.downcast_ref::<String>() {
            Some(string) => string.to_owned(),
            None => format!("{:?}", err),
        },
    }
}

/// Spawns a new thread to run the provided function `function` in a recoverable manner.
/// If the function `function` panics during execution, the panic will be caught, and the thread
/// will terminate without crashing the entire program.
///
/// # Parameters
/// - `function`: A function of type `function` to be executed in the spawned thread. It must implement `Fn()`, `Send`, `Sync`, and `'static` traits.
///     - `Fn()`: The function is callable with no arguments and no return value.
///     - `Send`: The function can be safely transferred across thread boundaries.
///     - `Sync`: The function can be shared across threads safely.
///     - `'static`: The function does not contain references to non-static data (i.e., data that lives beyond the function's scope).
///
/// # Returns
/// - A `JoinHandle<()>` representing the spawned thread. The thread can be joined later to wait for its completion.
///
///
/// # Panics
/// - This function itself will not panic, but the function `function` could panic during execution.
///   The panic will be caught, preventing the program from crashing.
#[inline]
pub fn recoverable_spawn<F>(function: F) -> JoinHandle<()>
where
    F: RecoverableFunction,
{
    spawn(|| {
        let _: SpawnResult = run_function(function);
    })
}

/// Spawns a recoverable function with an error-handling function in a new thread.
///
/// - `function`: The primary function to execute, implementing the `RecoverableFunction` trait.
/// - `error_handle_function`: A function to handle errors, implementing the `ErrorHandlerFunction` trait.
/// - Returns: A `JoinHandle<()>` that can be used to manage the spawned thread.
#[inline]
pub fn recoverable_spawn_with_error_handle<F, E>(
    function: F,
    error_handle_function: E,
) -> JoinHandle<()>
where
    F: RecoverableFunction,
    E: ErrorHandlerFunction,
{
    spawn(|| {
        let run_result: SpawnResult = run_function(function);
        if let Err(err) = run_result {
            let err_string: String = spawn_error_to_string(err);
            let _: SpawnResult = run_error_handle_function(error_handle_function, &err_string);
        }
    })
}
