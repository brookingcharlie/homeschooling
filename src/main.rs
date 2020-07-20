const NUM_PARTITIONS: usize = 3;

fn main() {
    match get_partitions(&[Task::new("A", 1), Task::new("B", 1), Task::new("C", 1)]) {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}

#[derive(Debug)]
struct Task {
    name: String,
    points: usize
}
impl Task {
    fn new(name: &str, points: usize) -> Task {
        Task{name: name.to_string(), points: points}
    }
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
    #[test_case(&[Task::new("A", 1), Task::new("B", 1), Task::new("C", 2)]; "indivisible tasks")]
    fn invalid(tasks: &[Task]) {
        assert!(get_partitions(tasks).is_none());
    }

    #[test_case(
        &[
            Task::new("A", 1),
            Task::new("B", 1),
            Task::new("C", 1)
        ];
        "three equal tasks"
    )]
    #[test_case(
        &[
            Task::new("A", 5),
            Task::new("B", 4),
            Task::new("C", 1),
            Task::new("D", 2),
            Task::new("E", 7),
            Task::new("F", 8),
            Task::new("G", 3)
        ];
        "given example"
    )]
    #[test_case(
        &[
            Task::new("A", 5),
            Task::new("B", 5),
            Task::new("C", 4),
            Task::new("D", 3),
            Task::new("E", 3),
            Task::new("F", 4),
            Task::new("G", 2),
            Task::new("H", 2),
            Task::new("G", 8)
        ];
        "trickier example"
    )]
    fn valid(tasks: &[Task]) {
        let partitions = get_partitions(tasks).unwrap();
        let correct =
            partitions.len() == NUM_PARTITIONS &&
            partitions.iter().all(|p| p.iter().map(|t| t.points).sum::<usize>() == tasks.iter().map(|t| t.points).sum::<usize>() / 3);
        assert!(correct, "incorrect partitions {:?}", partitions)
    }
}
