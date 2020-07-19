fn main() {
    match get_partitions(vec![1,1,1]) {
        None => println!("No"),
        Some(_) => println!("Yes"),
    }
}

fn get_partitions(xs: Vec<u32>) -> Option<Vec<Vec<u32>>> {
    if xs.len() < 3 || xs.iter().sum::<u32>() % 3 != 0  {
        return None
    }
    let mut partitions: Vec<Vec<u32>> = Vec::new();
    let partition1: Vec<u32> = Vec::new();
    let partition2: Vec<u32> = Vec::new();
    let partition3: Vec<u32> = Vec::new();
    partitions.push(partition1);
    partitions.push(partition2);
    partitions.push(partition3);
    Some(partitions)
}

#[cfg(test)]
mod tests {
    #[test]
    fn rejects_empty_set() {
        assert!(super::get_partitions(vec![]).is_none());
    }

    #[test]
    fn rejects_indivisible_elements() {
        assert!(super::get_partitions(vec![1,1,2]).is_none());
    }

    #[test]
    fn partitions_three_equal_elements() {
        let partitions = super::get_partitions(vec![1,1,1]).unwrap();
        assert_eq!(partitions.len(), 3);
        assert!(partitions.iter().all(|p| p.len() == 1 && p[0] == 1))
    }

    #[test]
    fn partitions_given_example() {
        let partitions = super::get_partitions(vec![5, 4, 1, 2, 7, 8, 3]).unwrap();
        assert_eq!(partitions.len(), 3);
        assert!(partitions.iter().all(|p| p.iter().sum::<u32>() == 10), "{:?}", partitions)
    }

    #[test]
    fn partitions_tricky_example() {
        let partitions = super::get_partitions(vec![5, 5, 4, 3, 3, 4, 2, 2, 8]).unwrap();
        assert_eq!(partitions.len(), 3);
        assert!(partitions.iter().all(|p| p.iter().sum::<u32>() == 12), "{:?}", partitions)
    }
}
