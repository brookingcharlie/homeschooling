const NUM_PARTITIONS: usize = 3;

#[derive(PartialEq, Debug)]
pub struct Task {
    pub name: String,
    pub points: usize
}
impl Task {
    pub fn new(name: &str, points: usize) -> Task {
        Task { name: name.to_string(), points: points }
    }
}

pub fn get_partitions<'a>(tasks: &'a [Task]) -> Option<Vec<Vec<&'a Task>>> {
    let total: usize = tasks.iter().map(|t| t.points).sum();
    if tasks.len() < NUM_PARTITIONS || total % NUM_PARTITIONS != 0 {
        return None
    }
    build_partitions(tasks, total / NUM_PARTITIONS, &[&[]; NUM_PARTITIONS])
}

fn build_partitions<'a>(tasks: &'a [Task], target: usize, partitions: &[&[&'a Task]; NUM_PARTITIONS]) -> Option<Vec<Vec<&'a Task>>> {
    if partitions.iter().all(|p| p.iter().map(|t| t.points).sum::<usize>() == target) {
        return Some(partitions.iter().map(|p| p.to_vec()).collect())
    }
    if tasks.len() == 0 {
        return None
    }
    for (i, partition) in partitions.iter().enumerate() {
        if partition.iter().map(|t| t.points).sum::<usize>() + tasks[0].points <= target {
            let mut new_partition = partition.to_vec(); new_partition.push(&tasks[0]);
            let mut new_partitions = partitions.clone(); new_partitions[i] = &new_partition;
            let result = build_partitions(&tasks[1..], target, &new_partitions);
            if result.is_some() {
                return result
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(&[]; "empty set")]
    #[test_case(&[5]; "one task")]
    #[test_case(&[6, 5]; "two tasks")]
    #[test_case(&[1, 1, 2]; "indivisible tasks")]
    fn does_not_partition_uneven_tasks(points_list: &[usize]) {
        assert!(get_partitions(&build_tasks(points_list)).is_none());
    }

    #[test_case(&[1, 1, 1]; "three equal tasks")]
    #[test_case(&[5, 4, 1, 2, 7, 8, 3]; "given example")]
    #[test_case(&[5, 5, 4, 3, 3, 4, 2, 2, 8]; "trickier example")]
    fn partitions_even_tasks(points_list: &[usize]) {
        let tasks = build_tasks(points_list);
        let partitions = get_partitions(&tasks).unwrap();
        assert_eq!(partitions.len(), NUM_PARTITIONS, "incorrect number of partitions {:?}", partitions);
        let target_sum = tasks.iter().map(|t| t.points).sum::<usize>() / NUM_PARTITIONS;
        let sums_correct = partitions.iter().all(|p| p.iter().map(|t| t.points).sum::<usize>() == target_sum);
        assert!(sums_correct, "incorrect partition sums {:?}", partitions)
    }

    fn build_tasks(points_list: &[usize]) -> Vec<Task> {
        points_list.iter().enumerate().map(|(i, points)| Task::new(&format!("Task {}", i), *points)).collect()
    }
}
