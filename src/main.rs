use std::thread;

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (_tx, rx) = oneshot::channel::<()>();

    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create runtime.");

        rt.block_on(async move {
            let _ = rx.await;
        });
    });
}
