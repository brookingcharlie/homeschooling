mod core;
mod io;

use std::io::{stdin, prelude::*};
use crate::{core::*, io::*};

fn main() {
    let lines: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let tasks: Vec<Task> = parse_tasks(&lines);
    match get_partitions(&tasks) {
        Some(partitions) => {
            println!("Yes");
            println!("");
            println!("{}", describe_partitions(&partitions).join("\n"));
        },
        None => println!("No")
    }
}
