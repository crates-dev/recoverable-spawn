use super::*;
use crate::*;
use task::JoinError;
use tokio::*;

/// Executes a recoverable function within a panic-safe context.
///
/// - `func`: A function implementing the `AsyncRecoverableFunction` trait.
/// - Returns: A `AsyncSpawnResult` indicating the success or failure of the function execution.
pub async fn async_run_function<F: AsyncRecoverableFunction>(func: F) -> AsyncSpawnResult {
    set_hook(Box::new(move |_| {}));
    let func = async move {
        func.call().await;
    };
    return tokio::spawn(func).await;
}

/// Executes an error-handling function with a given error message within a panic-safe context.
///
/// - `func`: A function implementing the `AsyncErrorHandlerFunction` trait.
/// - `error`: A string slice representing the error message.
/// - Returns: A `AsyncSpawnResult` indicating the success or failure of the error-handling function execution.
pub async fn async_run_error_handle_function<E: AsyncErrorHandlerFunction>(
    func: E,
    error: Arc<String>,
) -> AsyncSpawnResult {
    set_hook(Box::new(move |_| {}));
    let func = async move {
        func.call(error.clone()).await;
    };
    return tokio::spawn(func).await;
}

/// Converts a panic-captured error value into a string.
///
/// - `err`: The captured error value, of type `&JoinError `.
/// - Returns: A string representation of the error value.
pub fn tokio_error_to_string(err: &JoinError) -> String {
    err.to_string()
}

/// Spawns a new thread to run the provided function `function` in a recoverable manner.
/// If the function `function` panics during execution, the panic will be caught, and the thread
/// will terminate without crashing the entire program.
///
/// # Parameters
/// - `function`: A function of type `function` to be executed in the spawned thread. It must implement `FnOnce()`, `Send`, `Sync`, and `'static` traits.
///     - `FnOnce()`: The function is callable with no arguments and no return value.
///     - `Send`: The function can be safely transferred across thread boundaries.
///     - `Sync`: The function can be shared across threads safely.
///     - `'static`: The function does not contain references to non-static data (i.e., data that lives beyond the function's scope).
///
/// # Returns
/// - A `AsyncSpawnResult`
///
///
/// # Panics
/// - This function itself will not panic, but the function `function` could panic during execution.
///   The panic will be caught, preventing the program from crashing.
pub async fn async_recoverable_spawn<F>(function: F) -> AsyncSpawnResult
where
    F: AsyncRecoverableFunction,
{
    async_run_function(function).await
}

/// Spawns a recoverable function with an error-handling function in a new thread.
///
/// - `function`: The primary function to execute, implementing the `AsyncRecoverableFunction` trait.
/// - `error_handle_function`: A function to handle errors, implementing the `AsyncErrorHandlerFunction` trait.
pub async fn async_recoverable_spawn_catch<F, E>(
    function: F,
    error_handle_function: E,
) -> AsyncSpawnResult
where
    F: AsyncRecoverableFunction,
    E: AsyncErrorHandlerFunction,
{
    let run_result: AsyncSpawnResult = async_run_function(function).await;
    if let Err(err) = run_result.as_ref() {
        let err_string: String = tokio_error_to_string(err);
        let _: AsyncSpawnResult =
            async_run_error_handle_function(error_handle_function, Arc::new(err_string)).await;
    }
    return run_result;
}

/// Spawns an asynchronous recoverable function, catches any errors with an error-handling function,
/// and ensures that a final function is always executed, regardless of whether an error occurred.
///
/// This function runs a series of operations in an asynchronous context, where:
/// - `function` is executed first. If it results in an error, the `error_handle_function` is called.
/// - After either the main function or the error handler finishes, the `finally` function is executed.
/// This guarantees that the `finally` function runs regardless of the success or failure of the main operation.
///
/// # Parameters
/// - `function`: The primary function to execute, which must implement the `AsyncRecoverableFunction` trait.
/// - `error_handle_function`: A function that handles errors, which must implement the `AsyncErrorHandlerFunction` trait.
/// - `finally`: A function that will be executed after the main function and error handler, which must implement the `AsyncRecoverableFunction` trait.
///
/// # Returns
/// - A `JoinHandle<()>` that can be used to manage the spawned thread, ensuring that all the functions execute
///   in a recoverable context and the final block always runs.
///
/// # Errors
/// - If the `function` fails, the `error_handle_function` is invoked. If this fails as well, it will not stop the execution
///   of the `finally` block.
/// - The final block (`finally`) is always executed, even if the main function (`function`) or the error handler (`error_handle_function`) fails.
/// Spawns an asynchronous recoverable function, catches any errors with an error-handling function,
/// and ensures that a final function is always executed, regardless of whether an error occurred.
///
/// This function runs a series of operations in an asynchronous context, where:
/// - `function` is executed first. If it results in an error, the `error_handle_function` is called.
/// - After either the main function or the error handler finishes, the `finally` function is executed.
/// This guarantees that the `finally` function runs regardless of the success or failure of the main operation.
///
/// # Parameters
/// - `function`: The primary function to execute, which must implement the `AsyncRecoverableFunction` trait.
/// - `error_handle_function`: A function that handles errors, which must implement the `AsyncErrorHandlerFunction` trait.
/// - `finally`: A function that will be executed after the main function and error handler, which must implement the `AsyncRecoverableFunction` trait.
///
/// # Returns
/// - A `AsyncSpawnResult`
///
/// # Errors
/// - If the `function` fails, the `error_handle_function` is invoked. If this fails as well, it will not stop the execution
///   of the `finally` block.
/// - The final block (`finally`) is always executed, even if the main function (`function`) or the error handler (`error_handle_function`) fails.
pub async fn async_recoverable_spawn_catch_finally<F, E, L>(
    function: F,
    error_handle_function: E,
    finally: L,
) -> AsyncSpawnResult
where
    F: AsyncRecoverableFunction,
    E: AsyncErrorHandlerFunction,
    L: AsyncRecoverableFunction,
{
    let run_result: AsyncSpawnResult = async_run_function(function).await;
    if let Err(err) = run_result.as_ref() {
        let err_string: String = tokio_error_to_string(err);
        let _: AsyncSpawnResult =
            async_run_error_handle_function(error_handle_function, Arc::new(err_string)).await;
    }
    let _: AsyncSpawnResult = async_run_function(finally).await;
    return run_result;
}
