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
fn test_recoverable_spawn_catch() {
    use crate::*;
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn_catch(
        move || {
            panic!("{}", msg);
        },
        |err| {
            println!("handle error => {}", err);
        },
    );
    let _ = handle.join();
}

#[test]
fn test_recoverable_spawn_catch_finally() {
    use crate::*;
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn_catch_finally(
        move || {
            panic!("{}", msg);
        },
        |err| {
            println!("handle error => {}", err);
            panic!("{}", err);
        },
        || {
            println!("finally");
        },
    );
    let _ = handle.join();
}
