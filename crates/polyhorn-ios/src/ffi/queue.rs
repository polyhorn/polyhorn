use dispatch::Queue;
use std::mem::drop;
use std::sync::{Arc, Mutex};

pub struct QueueBound<T>
where
    T: 'static,
{
    queue: Queue,
    inner: Arc<Mutex<Option<InnerQueueBound<T>>>>,
}

impl<T> QueueBound<T>
where
    T: 'static,
{
    pub fn new<F>(queue: Queue, initializer: F) -> QueueBound<T>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let inner = Arc::new(Mutex::new(None));

        queue.exec_async({
            let inner = inner.clone();

            move || {
                inner
                    .lock()
                    .unwrap()
                    .replace(InnerQueueBound(initializer()));
            }
        });

        QueueBound { queue, inner }
    }

    pub unsafe fn adopt(queue: Queue, value: T) -> QueueBound<T> {
        let inner = Arc::new(Mutex::new(Some(InnerQueueBound(value))));

        QueueBound { queue, inner }
    }

    pub fn with<F>(&self, task: F)
    where
        F: FnOnce(&mut T) + Send + 'static,
    {
        let inner = self.inner.clone();

        self.queue.exec_async(move || {
            if let Some(value) = inner.lock().unwrap().as_mut() {
                task(&mut value.0);
            }
        })
    }

    pub unsafe fn with_adopt<F, V>(&self, value: V, task: F)
    where
        V: 'static,
        F: FnOnce(&mut T, V) + Send + 'static,
    {
        let wrap = InnerQueueBound(value);
        let inner = self.inner.clone();

        self.queue.exec_async(move || {
            if let Some(value) = inner.lock().unwrap().as_mut() {
                task(&mut value.0, wrap.0);
            }
        });
    }
}

impl<T> Drop for QueueBound<T>
where
    T: 'static,
{
    fn drop(&mut self) {
        if let Some(value) = self.inner.lock().unwrap().take() {
            self.queue.exec_async(move || drop(value));
        }
    }
}

struct InnerQueueBound<T>(T);

unsafe impl<T> Send for InnerQueueBound<T> {}
