#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {
    // TODO: do something more clever
    naive_order(tasks)
}

fn sort_by_queue_order(tasks: Vec<Task>) -> Vec<Task> {
    let mut sorted = tasks.clone();
    sorted.sort_unstable_by_key(|task| task.queued_at);
    sorted
}

// TODO: Combine these functions w/ a *_by_key abstraction
fn get_first_queued_task<'a>(tasks: &'a Vec<Task>) -> Option<&'a Task> {
    if let Some(first_task) = tasks.first() {
        let mut min_time = first_task.queued_at;
        let mut min_ind: usize = 0;
        for (i, task) in tasks.iter().enumerate().skip(1) {
            if task.queued_at < min_time {
                min_time = task.queued_at;
                min_ind = i;
            }
        }
        let first_queued_task = tasks[min_ind];
        return Some(&first_queued_task)
    }
    None
}

fn get_shortest_task_ind(tasks: Vec<&Task>) -> Option<usize> {
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
    // let mut reverse_sorted_queue = sort_by_queue_order(tasks);
    // reverse_sorted_queue.reverse();

    // Tasks that have not yet been executed
    let mut remaining_tasks = tasks.clone();

    // Ids of tasks that have been executed so far
    let mut executed_ids = Vec::<u64>::new();
    // Tasks that have been queued so far
    let mut current_queue = Vec::<&Task>::new();

    if let Some(first_task) = get_first_queued_task(&tasks) {
        let mut current_task = first_task;
        let mut current_time = first_task.queued_at;
        let mut previous_time = current_time;
        let mut next_task: &Task;
        while remaining_tasks.len() > 0 {
            // Generate new queue
            current_time += current_task.execution_duration;
            let newly_queued_tasks = tasks.iter().filter(|&task| previous_time < task.queued_at && task.queued_at <= current_time);
            current_queue.extend(newly_queued_tasks);

            if let Some(next_task_ind) = get_shortest_task_ind(current_queue) {
                // If at least one new task has been queued while the previous one was executing
                // Get next task
                next_task = current_queue.remove(next_task_ind);
            } else {
                // Otherwise, get the next task that will be queued later
                // Safe to unwrap here because we know that remaining_tasks.len() > 0
                next_task = get_first_queued_task(&remaining_tasks).unwrap();
                
            }
            // Record that task has been executed
            executed_ids.push(next_task.id);

            // Prepare for next step
            current_task = next_task;
            previous_time = current_time;
        }
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


    #[test]
    fn queue_order_sorting() {
        let tasks = vec![
            Task { id: 42, queued_at: 5, execution_duration: 3 },
            Task { id: 43, queued_at: 2, execution_duration: 3 },
            Task { id: 44, queued_at: 0, execution_duration: 2 },
        ];

        let sorted = sort_by_queue_order(tasks);
        let sorted_inds: Vec<_> = sorted.iter().map(|task| task.id).collect();
        assert_eq!(sorted_inds, vec![44, 43, 42]);
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
