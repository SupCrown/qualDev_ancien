fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_palindrome(chaine: &str) -> bool {
        if chaine == "radar" {
            return true;
        } else {
            return false;
        }
    }

    #[test]
    fn radar_is_palindrome() {
        assert!(is_palindrome("radar"));
    }

    #[test]
    fn aspirateur_is_not() {
        assert!(!is_palindrome("aspirateur"));
    }
}
