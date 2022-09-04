use std::time::Duration;

use futures::channel::oneshot;
use tokio::time::timeout;

mod local_worker;
use local_worker::LocalWorker;

#[tokio::main]
async fn main() {
    let worker = LocalWorker::new().expect("failed to create worker");

    let (tx1, rx1) = oneshot::channel();
    // let (tx2, rx2) = oneshot::channel();

    worker.spawn_pinned(move || async move {
        tx1.send(std::thread::current().id())
            .expect("failed to send!");
    });

    // worker.spawn_pinned(move || async move {
    //     tx2.send(std::thread::current().id())
    //         .expect("failed to send!");
    // });

    timeout(Duration::from_secs(5), rx1)
        .await
        .expect("task timed out")
        .expect("failed to receive");
    // timeout(Duration::from_secs(5), rx2)
    //     .await
    //     .expect("task timed out")
    //     .expect("failed to receive");
}
