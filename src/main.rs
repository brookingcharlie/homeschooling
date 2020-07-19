fn main() {
    match get_partitions(&[1,1,1]) {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}

fn get_partitions(xs: &[u32]) -> Option<Vec<Vec<u32>>> {
    let total = sum(xs);
    if xs.len() < 3 || total % 3 != 0  {
        return None
    }
    let target = total / 3;
    let p1: Vec<u32> = Vec::new();
    let p2: Vec<u32> = Vec::new();
    let p3: Vec<u32> = Vec::new();
    build_partitions(xs, target, &p1, &p2, &p3)
}

fn build_partitions(xs: &[u32], target: u32, p1: &Vec<u32>, p2: &Vec<u32>, p3: &Vec<u32>) -> Option<Vec<Vec<u32>>> {
    if sum(&p1) == target && sum(&p2) == target && sum(&p3) == target {
        return Some(vec![p1.to_vec(), p2.to_vec(), p3.to_vec()])
    }
    if xs.len() == 0 {
        return None
    }
    if sum(&p1) + xs[0] <= target {
        let mut p1x: Vec<u32> = p1.clone();
        p1x.push(xs[0]);
        let ps1 = build_partitions(&xs[1..], target, &p1x, &p2, &p3);
        if ps1.is_some() {
            return ps1
        }
    }
    if sum(&p2) + xs[0] <= target {
        let mut p2x: Vec<u32> = p2.clone();
        p2x.push(xs[0]);
        let ps2 = build_partitions(&xs[1..], target, &p1, &p2x, &p3);
        if ps2.is_some() {
            return ps2
        }
    }
    if sum(&p3) + xs[0] <= target {
        let mut p3x: Vec<u32> = p3.clone();
        p3x.push(xs[0]);
        let ps3 = build_partitions(&xs[1..], target, &p1, &p2, &p3x);
        if ps3.is_some() {
            return ps3
        }
    }
    None
}

fn sum(xs: &[u32]) -> u32 { xs.iter().sum::<u32>() }

#[cfg(test)]
mod tests {
    #[test]
    fn i_can_slice_array() {
        let xs = [10, 20, 30];
        assert_eq!(xs[1..], [20, 30]);
    }

    #[test]
    fn i_can_clone_and_extend_vector() {
        let xs: Vec<u32> = vec![10, 20];
        let mut ys: Vec<u32> = xs.clone();
        ys.push(30);
        assert_eq!(ys[..], [10, 20, 30]);
    }

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
        assert_eq!(ps.len(), 3);
        assert!(ps.iter().all(|p| p.len() == 1 && p[0] == 1), "{:?}", ps)
    }

    #[test]
    fn partitions_given_example() {
        let ps = super::get_partitions(&[5, 4, 1, 2, 7, 8, 3]).unwrap();
        assert_eq!(ps.len(), 3);
        assert!(ps.iter().all(|p| super::sum(p) == 10), "{:?}", ps)
    }

    #[test]
    fn partitions_tricky_example() {
        let ps = super::get_partitions(&[5, 5, 4, 3, 3, 4, 2, 2, 8]).unwrap();
        assert_eq!(ps.len(), 3);
        assert!(ps.iter().all(|p| super::sum(p) == 12), "{:?}", ps)
    }
}
