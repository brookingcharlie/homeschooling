const NUM_PARTITIONS: usize = 3;

fn main() {
    let tasks = [
        Task { name: String::from("A"), points: 1 },
        Task { name: String::from("B"), points: 1 },
        Task { name: String::from("C"), points: 1 }
    ];
    match get_partitions(&tasks) {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}

#[derive(Debug)]
struct Task {
    name: String,
    points: usize
}

fn get_partitions<'a>(tasks: &'a [Task]) -> Option<Vec<Vec<&'a Task>>> {
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
    fn invalid(points: &[usize]) {
        assert!(get_partitions(&tasks_from_points(points)).is_none());
    }

    #[test_case(&[1, 1, 1]; "three equal tasks")]
    #[test_case(&[5, 4, 1, 2, 7, 8, 3]; "given example")]
    #[test_case(&[5, 5, 4, 3, 3, 4, 2, 2, 8]; "trickier example")]
    fn valid(points: &[usize]) {
        let tasks = tasks_from_points(points);
        let partitions = get_partitions(&tasks).unwrap();
        let correct =
            partitions.len() == NUM_PARTITIONS &&
            partitions.iter().all(|p| p.iter().map(|t| t.points).sum::<usize>() == tasks.iter().map(|t| t.points).sum::<usize>() / 3);
        assert!(correct, "incorrect partitions {:?}", partitions)
    }

    fn tasks_from_points(points: &[usize]) -> Vec<Task> {
        points.iter().map(|points| Task { name: String::from("Task"), points: *points }).collect()
    }
}
