use dispatch::Queue;
use std::mem::drop;
use std::sync::{Arc, Mutex};

/// Wraps an object that is bound to a specific libdispatch DispatchQueue. As a
/// result, the wrapped object can only be accessed from the specified queue and
/// will always be dropped from that queue (regardless of which thread the
/// wrapper is dropped by).
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
    /// Returns a new queue bound wrapper around a value that is asynchronously
    /// initialized using the given closure. The given closure is executed on
    /// the given queue.
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

    /// Adopts a value that is assumed to already live on the given queue.
    pub unsafe fn adopt(queue: Queue, value: T) -> QueueBound<T> {
        let inner = Arc::new(Mutex::new(Some(InnerQueueBound(value))));

        QueueBound { queue, inner }
    }

    /// Sends the given task to the queue and applies it to the value this
    /// instance wraps.
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

    /// Sends the given task and a value that is assumed to already live on the
    /// this queue, to the queue and applies it to the value this instance
    /// wraps.
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
