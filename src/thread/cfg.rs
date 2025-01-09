#[test]
fn test_recoverable_spawn() {
    use crate::*;
    let handle: JoinHandle<()> = recoverable_spawn(|| {
        panic!("test");
    });
    let _ = handle.join();
}
