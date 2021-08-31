use std::{cmp::Ordering, collections::{BinaryHeap}};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

/// Used to order tasks by descending execution_duration
/// Inspired by std::cmp::Reverse - https://doc.rust-lang.org/src/core/cmp.rs.html#584
#[derive(Debug)]
struct TaskDurationDesc<'a>(pub &'a Task);

impl<'a> PartialEq for TaskDurationDesc<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.execution_duration == other.0.execution_duration
    }
}

impl<'a> Eq for TaskDurationDesc<'a> {}

impl<'a> PartialOrd for TaskDurationDesc<'a> {
    #[inline]
    fn partial_cmp(&self, other: &TaskDurationDesc) -> Option<Ordering> {
        other.0.execution_duration.partial_cmp(&self.0.execution_duration)
    }
}

impl<'a> Ord for TaskDurationDesc<'a> {
    #[inline]
    fn cmp(&self, other: &TaskDurationDesc<'a>) -> Ordering {
        other.0.execution_duration.cmp(&self.0.execution_duration)
    }
}

/// Time complexity: O(n)
fn get_shortest_task_ind(tasks: &Vec<&Task>) -> Option<usize> {
    if let Some(first_task) = tasks.first() {
        let mut min_duration = first_task.execution_duration;
        let mut min_ind: usize = 0;
        // TC: O(n)
        for (i, task) in tasks.iter().enumerate().skip(1) {
            if task.execution_duration < min_duration {
                min_duration = task.execution_duration;
                min_ind = i;
            }
        }
        return Some(min_ind)
    }
    None
}

pub trait Scheduler<'a> {
    fn new(tasks: &'a[Task]) -> Self;
    fn execution_order(&mut self) -> Vec<u64>;
}

pub struct NaiveScheduler<'a> {
    pub current_time: u32,
    // Tasks that have been queued so far
    pub current_queue: Vec<&'a Task>,
    // Tasks that have not yet been queued
    pub unqueued_tasks: Vec<&'a Task>,
}

impl<'a> NaiveScheduler<'a> {
    /// Time complexity: O(1)
    fn unfinished(&self) -> bool {
        // TC: O(1) - https://stackoverflow.com/questions/49775759/what-is-the-runtime-complexity-of-veclen
        self.unqueued_tasks.len() > 0 || self.current_queue.len() > 0
    }

    /// Time complexity: O(n)
    fn get_next_task(&mut self) -> Option<&'a Task> {
        let next_task;
        // TC: O(n)
        if let Some(next_task_ind) = get_shortest_task_ind(&self.current_queue) {
            // If at least one new task has been queued while the previous one was executing,
            // get the shortest one.
            // TC: O(n)
            next_task = self.current_queue.remove(next_task_ind);
            self.current_time += next_task.execution_duration;
            Some(next_task)
        } else {
            // Otherwise, fast-forward to the next task that will be queued.
            // NOTE: This assumes unqueued_tasks are in reverse-chronological order
            // TC: O(1) - https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1689
            if let Some(next_task) = self.unqueued_tasks.pop() {
                self.current_time = next_task.queued_at + next_task.execution_duration;
                Some(next_task)
            } else {
                // There might be no unqueued tasks to pop
                None
            }
        }
    }

    /// Time complexity: O(n)
    fn get_new_tasks(&mut self) -> Vec<&'a Task> {
        // See drain_filter: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
        let mut new_tasks = Vec::<&Task>::new();

        // TC: O(n)
        while self.unqueued_tasks.len() > 0 {
            // NOTE: This assumes unqueued_tasks is in reverse-chronological order
            if self.unqueued_tasks.last().unwrap().queued_at <= self.current_time {
                // Okay to unwrap because unqueued_tasks.len() > 0
                // TC: O(1)
                let new_task = self.unqueued_tasks.pop().unwrap();
                // TC: O(1)
                new_tasks.push(new_task);
            } else {
                break;
            }
        }

        new_tasks
    }

    /// Time complexity: O(n)
    fn update_queue(&mut self) {
        // TC: O(n)
        let new_tasks = self.get_new_tasks();
        // TC: O(n)
        self.current_queue.extend(new_tasks);
    }
}

impl<'a> Scheduler<'a> for NaiveScheduler<'a> {
    /// Time complexity: O(n*log(n))
    fn new(tasks: &'a[Task]) -> Self {
        // convert from Vec<Task> to Vec<&Task>
        // TC: O(n)
        let mut unqueued_tasks: Vec<&Task> = tasks.iter().collect();
        // Sort unqueued tasks in reverse-chronological queue time for easy popping
        // Safe to unwrap because two u64s always have a partial ordering.
        // TC: O(n*log(n)) - https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
        unqueued_tasks.sort_unstable_by(|&a, &b| b.queued_at.partial_cmp(&a.queued_at).unwrap());

        Self {
            current_time: 0,
            current_queue: vec![],
            unqueued_tasks,
        }
    }

    // Time Complexity: O(n^2)
    fn execution_order(&mut self) -> Vec<u64> {
        // Ids of tasks that have been executed so far
        let mut executed_ids = Vec::<u64>::new();

        // Loop over each task that gets executed
        // TC: O(n^2)
        while self.unfinished() /* TC: O(1) */ {
            // Choose the next task to execute
            // Okay to unwrap because unqueued_tasks.len() > 0
            // TC: O(n)
            let next_task = self.get_next_task().unwrap();
            // Record that the task has been executed
            // TC: O(1) - https://doc.rust-lang.org/std/collections/index.html#sequences
            executed_ids.push(next_task.id);
            // Queue any tasks submitted during execution
            // TC: O(n)
            self.update_queue();
        }

        executed_ids
    }
}

struct CleverScheduler<'a> {
    pub current_time: u32,
    pub unqueued_tasks: Vec<&'a Task>,
    pub current_queue: BinaryHeap<TaskDurationDesc<'a>>
}

impl<'a> CleverScheduler<'a> {
    /// Time complexity: O(1)
    fn unfinished(&self) -> bool {
        // TC: O(1) - https://stackoverflow.com/questions/49775759/what-is-the-runtime-complexity-of-veclen
        self.unqueued_tasks.len() > 0 || self.current_queue.len() > 0
    }

    // Time complexity: O(n*log(n))
    fn queue_tasks_submitted_before(&mut self, time: u32) {
        // Index of first task to be popped = # of tasks not to pop
        // TC: O(log(n))
        let num_later_tasks = self.unqueued_tasks.partition_point(|&task| task.queued_at >= time);
        // Number of tasks to pop
        let num_new_tasks = self.unqueued_tasks.len() - num_later_tasks;
        // TC: O(n*log(n)) - I think?
        for _ in 0..num_new_tasks {
            // Okay to unwrap because we know we have enough tasks to pop
            // TC: O(1)
            let task = self.unqueued_tasks.pop().unwrap();
            // TC: O(log(n))
            self.current_queue.push(TaskDurationDesc(task));
        }
    }

    // Time complexity: O(log(n))
    fn get_next_task(&mut self) -> Option<&'a Task> {
        // TC: O(log(n))
        if let Some(TaskDurationDesc(task)) = self.current_queue.pop() {
            // Grab from the queue if possible
            self.current_time += task.execution_duration;
            Some(task)
        } else {
            // Otherwise, fast-forward to the next queued task
            // TC: O(1)
            if let Some(task) = self.unqueued_tasks.pop() {
                // There was another task to be queued
                self.current_time = task.queued_at + task.execution_duration;
                Some(task)
            } else {
                // Looks like we're done (no more tasks, queued or unqueued)
                None
            }
        }
    }

    // Time complexity: O(n*log(n))
    fn update_queue(&mut self) {
        // TC: O(n*log(n))
        self.queue_tasks_submitted_before(self.current_time);
    }
}

impl<'a> Scheduler<'a> for CleverScheduler<'a> {
    /// Time complexity: O(n*log(n))
    fn new(tasks: &'a[Task]) -> Self {
        // convert from Vec<Task> to Vec<&Task>
        // TC: O(n)
        let mut unqueued_tasks: Vec<&Task> = tasks.iter().collect();
        // Sort unqueued tasks in reverse-chronological queue time for easy popping
        // Safe to unwrap because two u64s always have a partial ordering.
        // TC: O(n*log(n)) - https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
        unqueued_tasks.sort_unstable_by(|&a, &b| b.queued_at.partial_cmp(&a.queued_at).unwrap());

        let current_queue = BinaryHeap::new();

        Self {
            current_time: 0,
            unqueued_tasks,
            current_queue,
        }
    }

    // Time Complexity: TODO
    fn execution_order(&mut self) -> Vec<u64> {
        // Ids of tasks that have been executed so far
        let mut executed_ids = Vec::<u64>::new();

        // Loop over each task that gets executed
        // TC: O(n*log(n)) (TODO - hopefully)
        while self.unfinished() /* TC: O(1) */ {
            // Choose the next task to execute
            // Okay to unwrap because the queue is not empty
            // TC: O(log(n))
            let next_task = self.get_next_task().unwrap();

            // Record that the task has been executed
            // TC: O(1) - https://doc.rust-lang.org/std/collections/index.html#sequences
            executed_ids.push(next_task.id);
            // Queue any tasks submitted during execution
            // TC: O(n*log(n))
            self.update_queue();
        }

        executed_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_queue_order() {
        let tasks = vec![
            Task { id: 42, queued_at: 5, execution_duration: 3 },
            Task { id: 43, queued_at: 2, execution_duration: 3 },
            Task { id: 44, queued_at: 0, execution_duration: 2 },
        ];

        let mut naive_scheduler = NaiveScheduler::new(&tasks);
        let mut clever_scheduler = CleverScheduler::new(&tasks);

        assert_eq!(naive_scheduler.execution_order(), vec![44, 43, 42]);
        assert_eq!(clever_scheduler.execution_order(), vec![44, 43, 42]);
    }

    #[test]
    fn accepts_slice_arg() {
        let tasks = vec![
            Task { id: 42, queued_at: 5, execution_duration: 3 },
            Task { id: 43, queued_at: 2, execution_duration: 3 },
            Task { id: 44, queued_at: 0, execution_duration: 2 },
        ];

        let mut naive_scheduler = NaiveScheduler::new(tasks.as_slice());
        let mut clever_scheduler = CleverScheduler::new(&tasks);

        assert_eq!(naive_scheduler.execution_order(), vec![44, 43, 42]);
        assert_eq!(clever_scheduler.execution_order(), vec![44, 43, 42]);
    }


    // TODO: if two tasks are available with same duration, take the one queued first

    #[test]
    fn two_items_queued_at_once() {
        // 0: #42 is queued
        // 0: #42 is started
        // 1: #43 is queued
        // 2: #44 is queued
        // 3: #42 is finished
        // 3: #44 is started (it is queued and has a lower execution_duration than #43)
        // 5: #44 is finished
        // 5: #43 is started
        // 8: #43 is finished

        let tasks = vec![
            Task { id: 42, queued_at: 0, execution_duration: 3 },
            Task { id: 43, queued_at: 1, execution_duration: 3 },
            Task { id: 44, queued_at: 2, execution_duration: 2 },
        ];

        let mut naive_scheduler = NaiveScheduler::new(&tasks);
        let mut clever_scheduler = CleverScheduler::new(&tasks);

        assert_eq!(naive_scheduler.execution_order(), vec![42, 44, 43]);
        assert_eq!(clever_scheduler.execution_order(), vec![42, 44, 43]);
    }
}
