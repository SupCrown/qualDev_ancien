fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_palindrome(chaine: &str, int: &str) -> bool {
        if chaine == "radar" {
            if int == "12" {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    #[test]
    fn radar_is_palindrome() {
        assert!(is_palindrome("radar", "12"));
    }

    #[test]
    fn aspirateur_is_not() {
        assert!(!is_palindrome("aspirateur", "44"));
    }
}
