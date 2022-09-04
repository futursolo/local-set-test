use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
use std::{io, thread};

use std::sync::atomic::{AtomicUsize, Ordering};

use futures::channel::mpsc::UnboundedSender;
use futures::stream::StreamExt;
use tokio::task::{spawn_local, LocalSet};

type SpawnTask = Box<dyn Send + FnOnce()>;

thread_local! {
    static TASK_COUNT: RefCell<Option<Arc<AtomicUsize>>> = RefCell::new(None);
    static LOCAL_SET: LocalSet = LocalSet::new();
}

pub(crate) struct LocalWorker {
    task_count: Arc<AtomicUsize>,
    tx: UnboundedSender<SpawnTask>,
}

impl LocalWorker {
    pub fn new() -> io::Result<Self> {
        let (tx, mut rx) = futures::channel::mpsc::unbounded::<SpawnTask>();

        let task_count: Arc<AtomicUsize> = Arc::default();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        {
            let task_count = task_count.clone();
            thread::Builder::new().spawn(move || {
                TASK_COUNT.with(move |m| {
                    *m.borrow_mut() = Some(task_count);
                });

                LOCAL_SET.with(|local_set| {
                    local_set.block_on(&rt, async move {
                        while let Some(m) = rx.next().await {
                            m();
                        }
                    });
                });
            })?;
        }

        Ok(Self { task_count, tx })
    }

    pub fn task_count(&self) -> usize {
        self.task_count.load(Ordering::Acquire)
    }

    pub fn spawn_pinned<F, Fut>(&self, f: F)
    where
        F: 'static + Send + FnOnce() -> Fut,
        Fut: 'static + Future<Output = ()>,
    {
        let guard = LocalJobCountGuard::new(self.task_count.clone());

        // We ignore the result upon a failure, this can never happen unless the runtime is
        // exiting which all instances of Runtime will be dropped at that time and hence cannot
        // spawn pinned tasks.
        let _ = self.tx.unbounded_send(Box::new(move || {
            spawn_local(async move {
                let _guard = guard;

                f().await;
            });
        }));
    }
}

pub struct LocalJobCountGuard(Arc<AtomicUsize>);

impl LocalJobCountGuard {
    fn new(inner: Arc<AtomicUsize>) -> Self {
        inner.fetch_add(1, Ordering::AcqRel);
        LocalJobCountGuard(inner)
    }
}

impl Drop for LocalJobCountGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}
