#[test]
fn test_async_recoverable_spawn() {
    use crate::{JoinHandle, r#async::*};
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn(move || async move {
        panic!("{}", msg);
    });
    let _ = handle.join();
}

#[test]
fn test_async_recoverable_spawn_catch() {
    use crate::{JoinHandle, r#async::*};
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn_catch(
        move || async move {
            panic!("{}", msg);
        },
        move |err| async move {
            println!("handle error => {}", err);
        },
    );
    let _ = handle.join();
}

#[test]
fn test_async_recoverable_spawn_catch_finally() {
    use crate::{JoinHandle, r#async::*};
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn_catch_finally(
        move || async move {
            panic!("{}", msg);
        },
        move |err| async move {
            println!("handle error => {}", err);
            panic!("{}", err);
        },
        move || async move {
            println!("finally");
        },
    );
    let _ = handle.join();
}

#[tokio::test]
async fn test_async_async_recoverable_spawn() {
    use crate::r#async::*;
    let msg: &str = "test";
    async_recoverable_spawn(move || async move {
        panic!("{}", msg);
    })
    .await;
}

#[tokio::test]
async fn test_async_async_recoverable_spawn_catch() {
    use crate::r#async::*;
    let msg: &str = "test";
    async_recoverable_spawn_catch(
        move || async move {
            panic!("{}", msg);
        },
        move |err| async move {
            println!("async handle error => {}", err);
        },
    )
    .await;
}

#[tokio::test]
async fn test_async_async_recoverable_spawn_catch_finally() {
    use crate::r#async::*;
    let msg: &str = "test";
    async_recoverable_spawn_catch_finally(
        move || async move {
            panic!("{}", msg);
        },
        move |err| async move {
            println!("async handle error => {}", err);
            panic!("{}", err);
        },
        move || async move {
            println!("finally");
        },
    )
    .await;
}

#[test]
fn test_recoverable_spawn() {
    use crate::{JoinHandle, r#sync::*};
    let msg: &str = "test";
    let handle: JoinHandle<()> = recoverable_spawn(move || {
        panic!("{}", msg);
    });
    let _ = handle.join();
}

#[test]
fn test_recoverable_spawn_catch() {
    use crate::{JoinHandle, r#sync::*};
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
    use crate::{JoinHandle, r#sync::*};
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
