#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

// Should operate on anything that can be transformed into an iterator over tasks
pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {
    // TODO: do something more clever
    naive_order(tasks)
}

// TODO: Combine these functions w/ a *_by_key abstraction
fn get_first_queued_task<'a>(tasks: &mut Vec<&'a Task>) -> Option<&'a Task> {
    if let Some(first_task) = tasks.first() {
        let mut min_time = first_task.queued_at;
        let mut min_ind: usize = 0;
        for (i, task) in tasks.iter().enumerate().skip(1) {
            if task.queued_at < min_time {
                min_time = task.queued_at;
                min_ind = i;
            }
        }
        let first_queued_task = tasks.remove(min_ind);
        return Some(&first_queued_task)
    }
    None
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

fn get_new_tasks<'a>(unqueued_tasks: &mut Vec<&'a Task>, current_time: u32) -> Vec<&'a Task> {
    // See drain_filter: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
    let mut i = 0;
    let mut new_tasks = Vec::<&Task>::new();
    while i < unqueued_tasks.len() {
        if unqueued_tasks[i].queued_at <= current_time {
            new_tasks.push(unqueued_tasks.remove(i));
        } else {
            i += 1;
        }
    }
    new_tasks
}

fn update_queue<'a>(current_queue: &mut Vec<&'a Task>, unqueued_tasks: &mut Vec<&'a Task>, current_time: u32) {
    let new_tasks = get_new_tasks(unqueued_tasks, current_time);
    current_queue.extend(new_tasks);
}

fn get_next_task_from_queue<'a>(current_queue: &mut Vec<&'a Task>, unqueued_tasks: &mut Vec<&'a Task>, current_time: &mut u32) -> &'a Task {
    let next_task;
    if let Some(next_task_ind) = get_shortest_task_ind(&current_queue) {
        // If at least one new task has been queued while the previous one was executing,
        // get the shortest one.
        next_task = current_queue.remove(next_task_ind);
        *current_time += next_task.execution_duration;
    } else {
        // Otherwise, fast-forward to the next task that will be queued.
        // Safe to unwrap here because we know that unqueued_tasks.len() > 0
        next_task = get_first_queued_task(unqueued_tasks).unwrap();
        *current_time = next_task.queued_at + next_task.execution_duration;
    }

    next_task
}

pub fn naive_order(tasks: Vec<Task>) -> Vec<u64> {
    // Tasks that have not yet been executed
    // convert from Vec<Task> to Vec<&Task>
    let mut unqueued_tasks: Vec<&Task> = tasks.iter().collect();

    // Ids of tasks that have been executed so far
    let mut executed_ids = Vec::<u64>::new();
    // Tasks that have been queued so far
    let mut current_queue = Vec::<&Task>::new();

    if tasks.len() == 0 {
        return vec![];
    }

    // Initialize loop variables
    let mut current_time: u32 = 0;

    // Loop over each task that gets executed
    while unqueued_tasks.len() > 0 || current_queue.len() > 0 {
        // Choose the next task to execute
        let next_task = get_next_task_from_queue(&mut current_queue, &mut unqueued_tasks, &mut &mut current_time);
        // Record that the task has been executed
        executed_ids.push(next_task.id);
        // Queue any tasks submitted during execution
        update_queue(&mut current_queue, &mut unqueued_tasks, current_time);
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
