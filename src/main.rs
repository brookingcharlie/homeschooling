const NUM_PARTITIONS: usize = 3;

fn main() {
    match get_partitions(&[1,1,1]) {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}

fn get_partitions(xs: &[usize]) -> Option<Vec<Vec<usize>>> {
    let total: usize = xs.iter().sum();
    if xs.len() < NUM_PARTITIONS || total % NUM_PARTITIONS != 0  {
        return None
    }
    build_partitions(xs, total / NUM_PARTITIONS, vec![Vec::new(); NUM_PARTITIONS])
}

fn build_partitions(xs: &[usize], target: usize, ps: Vec<Vec<usize>>) -> Option<Vec<Vec<usize>>> {
    if ps.iter().all(|p| p.iter().sum::<usize>() == target) {
        return Some(ps.to_vec())
    }
    if xs.len() == 0 {
        return None
    }
    for (i, p) in ps.iter().enumerate() {
        if p.iter().sum::<usize>() + xs[0] <= target {
            let mut px: Vec<usize> = p.to_vec(); px.push(xs[0]);
            let mut psx: Vec<Vec<usize>> = ps.to_vec(); psx[i] = px;
            let result = build_partitions(&xs[1..], target, psx);
            if result.is_some() {
                return result
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
        let ps = super::get_partitions(&[1,1,1]).unwrap();
        assert_eq!(ps.len(), super::NUM_PARTITIONS);
        assert!(ps.iter().all(|p| p.len() == 1 && p[0] == 1), "{:?}", ps)
    }

    #[test]
    fn partitions_given_example() {
        let ps = super::get_partitions(&[5, 4, 1, 2, 7, 8, 3]).unwrap();
        assert_eq!(ps.len(), super::NUM_PARTITIONS);
        assert!(ps.iter().all(|p| p.iter().sum::<usize>() == 10), "{:?}", ps)
    }

    #[test]
    fn partitions_tricky_example() {
        let ps = super::get_partitions(&[5, 5, 4, 3, 3, 4, 2, 2, 8]).unwrap();
        assert_eq!(ps.len(), super::NUM_PARTITIONS);
        assert!(ps.iter().all(|p| p.iter().sum::<usize>() == 12), "{:?}", ps)
    }
}
