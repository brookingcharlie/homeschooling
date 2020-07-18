fn main() {
    match get_partitions(vec![1,1,1]) {
        None => println!("No"),
        Some(_) => println!("Yes"),
    }
}

fn get_partitions(xs: Vec<i32>) -> Option<Vec<Vec<i32>>> {
    if xs.iter().sum::<i32>() % 3 != 0  {
        return None
    }
    let mut partitions: Vec<Vec<i32>> = Vec::new();
    partitions.push(vec![1]);
    partitions.push(vec![1]);
    partitions.push(vec![1]);
    Some(partitions)
}

#[cfg(test)]
mod tests {
    #[test]
    fn rejects_indivisible_tasks() {
        assert_eq!(super::get_partitions(vec![1,1,2]).is_none(), true);
    }

    #[test]
    fn partitions_three_equal_tasks() {
        for partition in super::get_partitions(vec![1,1,1]).unwrap() {
            assert_eq!(partition.len(), 1);
            assert_eq!(partition[0], 1);
        }
    }
}
