fn main() {
    let name = "world";
    println!("{}", get_message(name));
}

fn get_message(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(super::get_message("foo"), "Hello, foo!");
    }
}
