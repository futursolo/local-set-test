use std::thread;
use std::time::Duration;

use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::sleep;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

async fn sleep_once() {
    sleep(Duration::ZERO).await;
}

#[tokio::main]
async fn main() {
    let rt = Runtime::new().expect("failed to create runtime.");

    thread::spawn(move || {
        LOCAL_SET.with(move |local_set| {
            local_set.spawn_local(sleep_once());
            local_set.block_on(&rt, sleep_once());
        });
    });
}
