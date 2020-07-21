const NUM_PARTITIONS: usize = 3;

#[derive(PartialEq, Debug)]
pub struct Task {
    pub name: String,
    pub points: usize
}

impl Task {
    pub fn new(name: &str, points: usize) -> Task {
        Task{name: name.to_string(), points: points}
    }
}

pub fn get_partitions<'a>(tasks: &'a [Task]) -> Option<Vec<Vec<&'a Task>>> {
    let total: usize = tasks.iter().map(|t| t.points).sum();
    if tasks.len() < NUM_PARTITIONS || total % NUM_PARTITIONS != 0  {
        return None
    }
    build_partitions(tasks, total / NUM_PARTITIONS, vec![Vec::new(); NUM_PARTITIONS])
}

fn build_partitions<'a>(tasks: &'a [Task], target: usize, partitions: Vec<Vec<&'a Task>>) -> Option<Vec<Vec<&'a Task>>> {
    if partitions.iter().all(|partition| partition.iter().map(|t| t.points).sum::<usize>() == target) {
        return Some(partitions.to_vec())
    }
    if tasks.len() == 0 {
        return None
    }
    for (i, partition) in partitions.iter().enumerate() {
        if partition.iter().map(|t| t.points).sum::<usize>() + tasks[0].points <= target {
            let mut new_partition: Vec<&Task> = partition.to_vec();
            new_partition.push(&tasks[0]);
            let mut new_partitions: Vec<Vec<&Task>> = partitions.to_vec();
            new_partitions[i] = new_partition;
            match build_partitions(&tasks[1..], target, new_partitions) {
                result @ Some(_) => return result,
                None => continue
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
    #[test_case(&[1, 1, 2]; "indivisible tasks")]
    fn invalid(points_list: &[usize]) {
        assert!(get_partitions(&tasks_from_points(points_list)).is_none());
    }

    #[test_case(&[1, 1, 1]; "three equal tasks")]
    #[test_case(&[5, 4, 1, 2, 7, 8, 3]; "given example")]
    #[test_case(&[5, 5, 4, 3, 3, 4, 2, 2, 8]; "trickier example")]
    fn valid(points_list: &[usize]) {
        let tasks = tasks_from_points(points_list);
        let partitions = get_partitions(&tasks).unwrap();
        assert_eq!(partitions.len(), NUM_PARTITIONS, "incorrect number of partitions {:?}", partitions);
        let target_sum = tasks.iter().map(|t| t.points).sum::<usize>() / NUM_PARTITIONS;
        let correct_sums = partitions.iter().all(|p| p.iter().map(|t| t.points).sum::<usize>() == target_sum);
        assert!(correct_sums, "incorrect partition sums {:?}", partitions)
    }

    fn tasks_from_points(points_list: &[usize]) -> Vec<Task> {
        points_list.iter().map(|points| Task::new("Task", *points)).collect()
    }
}
