fn main() {

}

fn is_palindrome(chaine: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radar_is_palindrome() {
        assert!(is_palindrome("radar"));
    }
}


