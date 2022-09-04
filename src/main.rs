use std::thread;
use std::time::Duration;

use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::sleep;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

#[tokio::main]
async fn main() {
    let rt = Runtime::new().expect("failed to create runtime.");

    thread::spawn(move || {
        LOCAL_SET.with(move |local_set| {
            local_set.block_on(&rt, async move {
                sleep(Duration::ZERO).await;
            });
        });
    });
}
