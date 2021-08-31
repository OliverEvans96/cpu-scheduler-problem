#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

struct Scheduler<'a> {
    pub current_time: u32,
    // Tasks that have been queued so far
    pub current_queue: Vec<&'a Task>,
    // Tasks that have not yet been queued
    pub unqueued_tasks: Vec<&'a Task>,
}

impl<'a> Scheduler<'a> {
    pub fn new(tasks: &'a Vec<Task>) -> Self {
        // convert from Vec<Task> to Vec<&Task>
        let mut unqueued_tasks: Vec<&Task> = tasks.iter().collect();
        // Sort unqueued tasks in reverse-chronological queue time for easy popping
        // Safe to unwrap because two u64s always have a partial ordering.
        unqueued_tasks.sort_unstable_by(|&a, &b| b.queued_at.partial_cmp(&a.queued_at).unwrap());

        Self {
            current_time: 0,
            current_queue: vec![],
            unqueued_tasks,
        }
    }

    pub fn unfinished(&self) -> bool {
        self.unqueued_tasks.len() > 0 || self.current_queue.len() > 0
    }

    // fn get_next_task_from_queue<'a>(current_queue: &mut Vec<&'a Task>, unqueued_tasks: &mut Vec<&'a Task>, current_time: &mut u32) -> &'a Task {
    pub fn get_next_task(&mut self) -> &'a Task {
        let next_task;
        if let Some(next_task_ind) = get_shortest_task_ind(&self.current_queue) {
            // If at least one new task has been queued while the previous one was executing,
            // get the shortest one.
            next_task = self.current_queue.remove(next_task_ind);
            self.current_time += next_task.execution_duration;
        } else {
            // Otherwise, fast-forward to the next task that will be queued.
            // Safe to unwrap here because we know that unqueued_tasks.len() > 0
            // NOTE: This assumes unqueued_tasks are in reverse-chronological order
            next_task = self.unqueued_tasks.pop().unwrap();
            self.current_time = next_task.queued_at + next_task.execution_duration;
        }

        next_task
    }

    fn get_new_tasks(&mut self) -> Vec<&'a Task> {
        // See drain_filter: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
        let mut i = 0;
        let mut new_tasks = Vec::<&Task>::new();
        while i < self.unqueued_tasks.len() {
            if self.unqueued_tasks[i].queued_at <= self.current_time {
                new_tasks.push(self.unqueued_tasks.remove(i));
            } else {
                i += 1;
            }
        }
        new_tasks
    }

    pub fn update_queue(&mut self) {
        let new_tasks = self.get_new_tasks();
        self.current_queue.extend(new_tasks);
    }

}

// Should operate on anything that can be transformed into an iterator over tasks
pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {
    // TODO: do something more clever
    naive_order(tasks)
}

fn get_shortest_task_ind(tasks: &Vec<&Task>) -> Option<usize> {
    if let Some(first_task) = tasks.first() {
        let mut min_duration = first_task.execution_duration;
        let mut min_ind: usize = 0;
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


pub fn naive_order(tasks: Vec<Task>) -> Vec<u64> {
    // Do nothing if there are no tasks
    if tasks.len() == 0 {
        return vec![];
    }

    let mut scheduler = Scheduler::new(&tasks);

    // Ids of tasks that have been executed so far
    let mut executed_ids = Vec::<u64>::new();

    // Loop over each task that gets executed
    while scheduler.unfinished() {
        // Choose the next task to execute
        let next_task = scheduler.get_next_task();
        // Record that the task has been executed
        executed_ids.push(next_task.id);
        // Queue any tasks submitted during execution
        scheduler.update_queue();
    }

    executed_ids
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

        assert_eq!(execution_order(tasks), vec![44, 43, 42]);
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

        assert_eq!(execution_order(tasks), vec![42, 44, 43]);
    }
}
