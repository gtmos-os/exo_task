use crate::task::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

/// An async task executor that manages multiple tasks and their wakers.
/// Uses a combination of BTreeMap for task storage and ArrayQueue for scheduling.
pub struct Executor {
    /// Stores all tasks indexed by their TaskId
    tasks: BTreeMap<TaskId, Task>,
    /// Queue of task IDs that are ready to be polled
    task_queue: Arc<ArrayQueue<TaskId>>,
    /// Cache of wakers for each task to avoid recreation
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    /// Creates a new Executor with empty task collections and a fixed-size queue.
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// Spawns a new task into the executor.
    /// 
    /// # Arguments
    /// * `task` - The task to be executed
    ///
    /// # Panics
    /// * If a task with the same ID already exists
    /// * If the task queue is full
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    /// Runs the executor in an infinite loop, processing ready tasks
    /// and sleeping when idle.
    /// 
    /// # Returns
    /// Never returns (!) as it runs indefinitely
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    /// Processes all tasks currently in the task queue.
    /// Tasks that are Poll::Ready are removed, while Poll::Pending
    /// tasks remain in the executor.
    fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    /// Puts the executor to sleep if there are no tasks to process.
    /// When x86_64_support is enabled, uses CPU-specific sleep instructions.
    /// Otherwise it does nothing.
    fn sleep_if_idle(&self) {
        #[cfg(feature = "x86_64_support")]
        {
            use x86_64::instructions::interrupts::{self, enable_and_hlt};
    
            interrupts::disable();
            if self.task_queue.is_empty() {
                enable_and_hlt();
            } else {
                interrupts::enable();
            }
        }
    }
}

/// Provides the wake mechanism for tasks in the executor.
/// Uses an ArrayQueue to push tasks back into the ready queue.
struct TaskWaker {
    /// ID of the task this waker is associated with
    task_id: TaskId,
    /// Reference to the executor's task queue
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    /// Creates a new Waker for a specific task.
    /// 
    /// # Arguments
    /// * `task_id` - ID of the task to wake
    /// * `task_queue` - Queue to push the task ID into when woken
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    /// Pushes the task ID back into the queue, marking it as ready to run.
    /// 
    /// # Panics
    /// If the task queue is full
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    /// Wakes a task by consuming the waker
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    /// Wakes a task by reference
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}