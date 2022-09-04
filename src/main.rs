use std::thread;

use tokio::sync::mpsc;
use tokio::task::LocalSet;

type SpawnTask = Box<dyn Send + FnOnce()>;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

#[tokio::main]
async fn main() {
    let (_tx, mut rx) = mpsc::unbounded_channel::<SpawnTask>();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create runtime.");

        LOCAL_SET.with(|local_set| {
            local_set.block_on(&rt, async move {
                while let Some(m) = rx.recv().await {
                    m();
                }
            });
        });
    });
}
