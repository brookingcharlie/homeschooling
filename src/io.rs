use regex::Regex;
use crate::core::*;

pub fn parse_tasks(lines: &[String]) -> Vec<Task> {
    let regex = Regex::new(r"^(?P<name>[^:]+?)\s*:\s*(?P<points>[\d]+)\s*(points?)?$").unwrap();
    lines.iter()
        .map(|line| regex.captures(&line).unwrap())
        .map(|caps| Task::new(&caps["name"], caps["points"].parse::<usize>().unwrap()))
        .collect()
}

pub fn describe_partitions(partitions: &[Vec<&Task>]) -> Vec<String> {
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
    use super::*;

    #[test]
    fn parses_valid_lines() {
        let tasks = parse_tasks(&[
            String::from("Task A : 5"),
            String::from("Task Beta:13points"),
            String::from("Task 1/23 :1  point")
        ]);
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0], Task::new("Task A", 5));
        assert_eq!(tasks[1], Task::new("Task Beta", 13));
        assert_eq!(tasks[2], Task::new("Task 1/23", 1));
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_lines() {
        parse_tasks(&[String::from("Task A: 5"), String::from("Task B = 13")]);
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
