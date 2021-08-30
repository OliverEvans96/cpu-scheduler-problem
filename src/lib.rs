#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub queued_at: u32,
    pub execution_duration: u32,
}

pub fn execution_order(tasks: Vec<Task>) -> Vec<u64> {
    todo!()
}

fn sort_by_queue_order(tasks: Vec<Task>) -> Vec<Task> {
    let mut sorted = tasks.clone();
    sorted.sort_unstable_by_key(|task| task.queued_at);
    sorted
}

pub fn naive_order(tasks: Vec<Task>) -> Vec<u64> {
    let sorted = sort_by_queue_order(tasks);
    // TODO
    vec![1,2,3]
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
