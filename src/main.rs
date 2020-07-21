use std::io::stdin;
use std::io::prelude::*;
use regex::Regex;

const NUM_PARTITIONS: usize = 3;

fn main() {
    let lines: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let tasks: Vec<Task> = parse_tasks(&lines);
    match get_partitions(&tasks) {
        Some(partitions) => {
            println!("Yes");
            println!("\n");
            println!("{}", describe_partitions(&partitions).join("\n"));
        },
        None => println!("No")
    }
}

#[derive(PartialEq, Debug)]
struct Task {
    name: String,
    points: usize
}
impl Task {
    fn new(name: &str, points: usize) -> Task {
        Task{name: name.to_string(), points: points}
    }
}

fn parse_tasks(lines: &[String]) -> Vec<Task> {
    let regex = Regex::new(r"^(?P<name>[^:]+?)\s*:\s*(?P<points>[\d]+)$").unwrap();
    lines.iter()
        .map(|line| regex.captures(&line).unwrap())
        .map(|caps| Task::new(&caps["name"], caps["points"].parse::<usize>().unwrap()))
        .collect()
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

fn describe_partitions(partitions: &Vec<Vec<&Task>>) -> Vec<String> {
    partitions.iter().enumerate().map(|(i, partition)| {
        format!(
            "Child {}: {} = {} points",
            i + 1,
            partition.iter()
                .map(|t| format!("{} ({} {})", t.name, t.points, if t.points == 1 { "point" } else { "points" }))
                .collect::<Vec<String>>()
                .join(" + "),
            partition.iter().map(|t| t.points).sum::<usize>()
        )
    }).collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test]
    fn parses_valid_lines() {
        let tasks = parse_tasks(&[String::from("Task A : 5"), String::from("Task B:13")]);
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0], Task::new("Task A", 5));
        assert_eq!(tasks[1], Task::new("Task B", 13));
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_lines() {
        parse_tasks(&[String::from("Task A: 5"), String::from("Task B = 13")]);
    }

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

    #[test]
    fn describes_partitions() {
        let output: Vec<String> = describe_partitions(&vec![
            vec![&Task::new("Task D", 2), &Task::new("Task F", 8)],
            vec![&Task::new("Task A", 5), &Task::new("Task B", 4), &Task::new("Task C", 1)],
            vec![&Task::new("Task E", 7), &Task::new("Task G", 3)]
        ]);
        assert_eq!(output[0], "Child 1: Task D (2 points) + Task F (8 points) = 10 points");
        assert_eq!(output[1], "Child 2: Task A (5 points) + Task B (4 points) + Task C (1 point) = 10 points");
        assert_eq!(output[2], "Child 3: Task E (7 points) + Task G (3 points) = 10 points");
    }
}
