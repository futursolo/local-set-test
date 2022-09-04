use std::thread;

use tokio::sync::oneshot;
use tokio::task::LocalSet;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

#[tokio::main]
async fn main() {
    // holds runtime thread until end of main fn.
    let (_tx, rx) = oneshot::channel::<()>();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create runtime.");

        LOCAL_SET.with(|local_set| {
            local_set.block_on(&rt, async move {
                let _ = rx.await;
            });
        });
    });
}
