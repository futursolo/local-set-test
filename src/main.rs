use futures::channel::oneshot;

mod local_worker;
use local_worker::LocalWorker;

#[tokio::main]
async fn main() {
    let worker = LocalWorker::new().expect("failed to create worker");

    let (tx, rx) = oneshot::channel();

    worker.spawn_pinned(move || async move {
        tx.send(()).expect("failed to send!");
    });

    rx.await.expect("failed to receive");
}
