use multiset::HashMultiSet;

fn main() {
    for x in get_partition().iter() {
        println!("{}", x);
    }
}

fn get_partition() -> HashMultiSet<i32> {
    let mut result = HashMultiSet::new();
    result.insert(1);
    result.insert(2);
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = super::get_partition();
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(!result.contains(&3));
    }
}
