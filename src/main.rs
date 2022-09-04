use std::time::Duration;

use crate::rt::Runtime;
use futures::channel::oneshot;
use tokio::time::timeout;

mod local_worker;
mod rt;

// async fn sleep_once() {
//     sleep(Duration::ZERO).await;
// }

#[tokio::main]
async fn main() {
    let runtime = Runtime::new(2).expect("failed to create runtime.");

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    runtime.spawn_pinned(move || async move {
        tx1.send(std::thread::current().id())
            .expect("failed to send!");
    });

    runtime.spawn_pinned(move || async move {
        tx2.send(std::thread::current().id())
            .expect("failed to send!");
    });

    let result1 = timeout(Duration::from_secs(5), rx1)
        .await
        .expect("task timed out")
        .expect("failed to receive");
    let result2 = timeout(Duration::from_secs(5), rx2)
        .await
        .expect("task timed out")
        .expect("failed to receive");

    // first task and second task are not on the same thread.
    assert_ne!(result1, result2);
}
