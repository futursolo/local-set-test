use std::thread;

use tokio::runtime::Handle;
use tokio::sync::oneshot;
use tokio::task::LocalSet;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

#[tokio::main]
async fn main() {
    // holds runtime thread until end of main fn.
    let (_tx, rx) = oneshot::channel::<()>();
    let handle = Handle::current();

    thread::spawn(move || {
        LOCAL_SET.with(|local_set| {
            handle.block_on(local_set.run_until(async move {
                let _ = rx.await;
            }))
        });
    });
}
