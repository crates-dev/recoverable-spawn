#[tokio::test]
async fn test_async_recoverable_spawn() {
    use crate::r#async::*;
    let msg: &str = "test";
    let res: AsyncSpawnResult = async_recoverable_spawn(move || async move {
        panic!("{}", msg);
    })
    .await;
    println!("test_async_recoverable_spawn handle res {res:?}");
}

#[tokio::test]
async fn test_async_recoverable_spawn_catch() {
    use crate::r#async::*;
    let msg: &str = "test";
    let res: AsyncSpawnResult = async_recoverable_spawn_catch(
        move || async move {
            panic!("{}", msg);
        },
        move |err| async move {
            println!("async handle error => {err}");
        },
    )
    .await;
    println!("test_async_recoverable_spawn_catch handle res {res:?}");
}

#[tokio::test]
async fn test_async_recoverable_spawn_catch_finally() {
    use crate::r#async::*;
    let msg: &str = "test";
    let res: AsyncSpawnResult = async_recoverable_spawn_catch_finally(
        move || async move {
            panic!("{}", msg);
        },
        move |err| async move {
            println!("async handle error => {err}");
            panic!("{}", err);
        },
        move || async move {
            println!("finally");
        },
    )
    .await;
    println!("test_async_recoverable_spawn_catch_finally handle res {res:?}");
}

#[test]
fn test_recoverable_spawn() {
    use crate::r#sync::*;
    let msg: &str = "test";
    let res: SyncSpawnResult = recoverable_spawn(move || {
        panic!("{}", msg);
    });
    println!("test_recoverable_spawn handle res {res:?}");
}

#[test]
fn test_recoverable_spawn_catch() {
    use crate::r#sync::*;
    let msg: &str = "test";
    let res: SyncSpawnResult = recoverable_spawn_catch(
        move || {
            panic!("{}", msg);
        },
        |err| {
            println!("handle error => {err}");
        },
    );
    println!("test_recoverable_spawn_catch handle res {res:?}");
}

#[test]
fn test_recoverable_spawn_catch_finally() {
    use crate::r#sync::*;
    let msg: &str = "test";
    let res: SyncSpawnResult = recoverable_spawn_catch_finally(
        move || {
            panic!("{}", msg);
        },
        |err| {
            println!("handle error => {err}");
            panic!("{}", err);
        },
        || {
            println!("finally");
        },
    );
    println!("test_recoverable_spawn_catch_finally handle res {res:?}");
}
