use crate::task::Task;
use alloc::collections::VecDeque;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// A basic task executor that runs tasks in a FIFO queue.
/// This implementation uses a dummy waker that does nothing when woken.
pub struct SimpleExecutor {
    /// Queue of tasks waiting to be executed
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    /// Creates a new SimpleExecutor with an empty task queue.
    pub fn new() -> SimpleExecutor {
        SimpleExecutor {
            task_queue: VecDeque::new(),
        }
    }

    /// Adds a new task to the back of the task queue.
    /// 
    /// # Arguments
    /// * `task` - The task to be executed
    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }

    /// Runs all tasks in the queue until completion.
    /// Tasks that return Poll::Pending are pushed back to the queue.
    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {} // task done
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

/// Creates a RawWaker with no-op implementations of all required methods.
/// This is used for tasks that don't need to be woken up externally.
/// 
/// # Safety
/// The returned RawWaker does nothing when called, which is safe but may
/// lead to inefficient polling if tasks rely on waking mechanisms.
fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

/// Creates a Waker from the dummy RawWaker.
/// 
/// # Safety
/// This is safe because the dummy_raw_waker implements all required methods,
/// albeit as no-ops.
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}