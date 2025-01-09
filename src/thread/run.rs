use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};

/// Spawns a new thread to run the provided function `f` in a recoverable manner.
/// If the function `f` panics during execution, the panic will be caught, and the thread
/// will terminate without crashing the entire program.
///
/// # Parameters
/// - `f`: A function of type `F` to be executed in the spawned thread. It must implement `Fn()`, `Send`, `Sync`, and `'static` traits.
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
/// - This function itself will not panic, but the function `f` could panic during execution.
///   The panic will be caught, preventing the program from crashing.
pub fn recoverable_spawn<F>(f: F) -> JoinHandle<()>
where
    F: Fn() + Send + Sync + 'static,
{
    let f: Arc<RwLock<F>> = Arc::new(RwLock::new(f));
    spawn(move || {
        let _: Result<(), Box<dyn Any + Send>> = panic::catch_unwind(AssertUnwindSafe({
            let f: Arc<RwLock<F>> = Arc::clone(&f);
            move || {
                if let Ok(func) = f.read() {
                    func();
                }
            }
        }));
    })
}
