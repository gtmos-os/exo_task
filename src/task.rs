use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};

/// Represents an asynchronous task that can be executed by the executor.
/// Each task has a unique identifier and contains a pinned future.
pub struct Task {
    /// Unique identifier for the task
    pub(crate) id: TaskId,
    /// The actual future that will be polled to completion
    pub(crate) future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Creates a new task from a future.
    /// 
    /// # Arguments
    /// * `future` - Any future that returns () and has a static lifetime
    ///
    /// # Returns
    /// A new Task instance with a unique TaskId
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    /// Polls the internal future to make progress on the task.
    /// 
    /// # Arguments
    /// * `context` - The task context containing the waker
    ///
    /// # Returns
    /// Poll::Ready(()) when the future completes, or Poll::Pending if it's not ready
    pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

/// Represents a unique identifier for a task.
/// Implemented as a newtype pattern around u64 for type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(u64);

impl TaskId {
    /// Creates a new unique TaskId using an atomic counter.
    /// 
    /// Uses Relaxed ordering since the counter only needs to be unique,
    /// not synchronized with other operations.
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}