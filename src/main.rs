use std::thread;
use std::time::Duration;

use tokio::task::LocalSet;
use tokio::time::sleep;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

#[tokio::main]
async fn main() {
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create runtime.");

        LOCAL_SET.with(|local_set| {
            local_set.block_on(&rt, async move {
                sleep(Duration::ZERO).await;
            });
        });
    });
}
