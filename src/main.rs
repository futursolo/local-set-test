use std::future::Future;
use std::thread;

use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::oneshot;
use tokio::task::{spawn_local, LocalSet};

type SpawnTask = Box<dyn Send + FnOnce()>;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

pub(crate) struct LocalWorker {
    tx: UnboundedSender<SpawnTask>,
}

impl LocalWorker {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<SpawnTask>();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create runtime.");

        thread::spawn(move || {
            LOCAL_SET.with(|local_set| {
                local_set.block_on(&rt, async move {
                    while let Some(m) = rx.recv().await {
                        m();
                    }
                });
            });
        });

        Self { tx }
    }

    pub fn spawn_pinned<F, Fut>(&self, f: F)
    where
        F: 'static + Send + FnOnce() -> Fut,
        Fut: 'static + Future<Output = ()>,
    {
        let _ = self.tx.send(Box::new(move || {
            spawn_local(async move {
                f().await;
            });
        }));
    }
}

#[tokio::main]
async fn main() {
    let worker = LocalWorker::new();

    let (tx, rx) = oneshot::channel();

    worker.spawn_pinned(move || async move {
        tx.send(()).expect("failed to send!");
    });

    rx.await.expect("failed to receive");
}
