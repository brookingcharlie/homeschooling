const NUM_PARTITIONS: usize = 3;

fn main() {
    match get_partitions(&[1,1,1]) {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}

fn get_partitions(elements: &[usize]) -> Option<Vec<Vec<usize>>> {
    let total: usize = elements.iter().sum();
    if elements.len() < NUM_PARTITIONS || total % NUM_PARTITIONS != 0  {
        return None
    }
    build_partitions(elements, total / NUM_PARTITIONS, vec![Vec::new(); NUM_PARTITIONS])
}

fn build_partitions(elements: &[usize], target: usize, partitions: Vec<Vec<usize>>) -> Option<Vec<Vec<usize>>> {
    if partitions.iter().all(|partition| partition.iter().sum::<usize>() == target) {
        return Some(partitions.to_vec())
    }
    if elements.len() == 0 {
        return None
    }
    for (i, partition) in partitions.iter().enumerate() {
        if partition.iter().sum::<usize>() + elements[0] <= target {
            let mut new_partition: Vec<usize> = partition.to_vec();
            new_partition.push(elements[0]);
            let mut new_partitions: Vec<Vec<usize>> = partitions.to_vec();
            new_partitions[i] = new_partition;
            match build_partitions(&elements[1..], target, new_partitions) {
                result @ Some(_) => return result,
                None => continue
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn rejects_empty_set() {
        assert!(super::get_partitions(&[]).is_none());
    }

    #[test]
    fn rejects_indivisible_elements() {
        assert!(super::get_partitions(&[1,1,2]).is_none());
    }

    #[test]
    fn partitions_three_equal_elements() {
        let partitions = super::get_partitions(&[1,1,1]).unwrap();
        assert_eq!(partitions.len(), super::NUM_PARTITIONS);
        assert!(partitions.iter().all(|partition| partition.iter().sum::<usize>() == 1), "{:?}", partitions)
    }

    #[test]
    fn partitions_given_example() {
        let partitions = super::get_partitions(&[5, 4, 1, 2, 7, 8, 3]).unwrap();
        assert_eq!(partitions.len(), super::NUM_PARTITIONS);
        assert!(partitions.iter().all(|partition| partition.iter().sum::<usize>() == 10), "{:?}", partitions)
    }

    #[test]
    fn partitions_tricky_example() {
        let partitions = super::get_partitions(&[5, 5, 4, 3, 3, 4, 2, 2, 8]).unwrap();
        assert_eq!(partitions.len(), super::NUM_PARTITIONS);
        assert!(partitions.iter().all(|partition| partition.iter().sum::<usize>() == 12), "{:?}", partitions)
    }
}
