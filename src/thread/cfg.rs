#[test]
fn test_recoverable_spawn() {
    use crate::*;
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn(move || {
        panic!("{}", msg);
    });
    let _ = handle.join();
}

#[test]
fn test_recoverable_spawn_with_error_handle() {
    use crate::*;
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn_with_error_handle(
        move || {
            panic!("{}", msg);
        },
        |err| {
            println!("handle error => {}", err);
        },
    );
    let _ = handle.join();
}
