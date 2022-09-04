use std::thread;

use tokio::sync::oneshot;
use tokio::task::LocalSet;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

fn main() {
    // holds runtime thread until end of main fn.
    let (_tx, rx) = oneshot::channel::<()>();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create runtime.");

    thread::spawn(move || {
        LOCAL_SET.with(|local_set| {
            local_set.block_on(&rt, async move {
                let _ = rx.await;
            });
        });
    });
}
