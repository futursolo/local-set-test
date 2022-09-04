use std::future::Future;
use std::{io, thread};

use futures::channel::mpsc::UnboundedSender;
use futures::stream::StreamExt;
use tokio::task::{spawn_local, LocalSet};

type SpawnTask = Box<dyn Send + FnOnce()>;

thread_local! {
    static LOCAL_SET: LocalSet = LocalSet::new();
}

pub(crate) struct LocalWorker {
    tx: UnboundedSender<SpawnTask>,
}

impl LocalWorker {
    pub fn new() -> io::Result<Self> {
        let (tx, mut rx) = futures::channel::mpsc::unbounded::<SpawnTask>();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        thread::Builder::new().spawn(move || {
            LOCAL_SET.with(|local_set| {
                local_set.block_on(&rt, async move {
                    while let Some(m) = rx.next().await {
                        m();
                    }
                });
            });
        })?;

        Ok(Self { tx })
    }

    pub fn spawn_pinned<F, Fut>(&self, f: F)
    where
        F: 'static + Send + FnOnce() -> Fut,
        Fut: 'static + Future<Output = ()>,
    {
        // We ignore the result upon a failure, this can never happen unless the runtime is
        // exiting which all instances of Runtime will be dropped at that time and hence cannot
        // spawn pinned tasks.
        let _ = self.tx.unbounded_send(Box::new(move || {
            spawn_local(async move {
                f().await;
            });
        }));
    }
}
