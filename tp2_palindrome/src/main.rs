fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_palindrome(chaine: &str) -> bool {
        true
    }

    #[test]
    fn hello_is_palindrome() {
        assert!(is_palindrome("hello"));
    }

    #[test]
    fn aspirateur_is_not() {
        assert!(!is_palindrome("aspirateur"));
    }
}
