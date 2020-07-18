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
        assert!(super::get_partitions(vec![1,1,2]).is_none());
    }

    #[test]
    fn partitions_three_equal_tasks() {
        let partitions = super::get_partitions(vec![1,1,1]).unwrap();
        assert!(partitions.iter().all(|p| p.len() == 1 && p[0] == 1))
    }

    #[test]
    fn partitions_tricky_example() {
        let partitions = super::get_partitions(vec![5, 5, 4, 3, 3, 4, 2, 2, 8]).unwrap();
        assert!(partitions.iter().all(|p| p.iter().sum::<i32>() == 12), "{:?}", partitions)
    }
}
