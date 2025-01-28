extern crate unidecode;
use unidecode::unidecode;

fn main() {

}

fn is_palindrome(chaine: &str) -> bool {
    unidecode(chaine.trim().to_lowercase().chars().collect()) == unidecode(chaine.trim().chars().rev().collect::<String>())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radar_is_palindrome() {
        assert!(is_palindrome("radar"));
    }

    #[test]
    fn aspirateur_is_not_palindrome() {
        assert!(!is_palindrome("aspirateur"))
    }
}


