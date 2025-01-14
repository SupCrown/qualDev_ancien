

/// My first Rust program that display a personalized greeting
fn main() {
    let repetition = 4;
    println!("{}", greetings(repetition));
}

/// My first function, returns a friendly greeting
/// # Arguments
/// * `rep` - The number of 'o' to add to the greeting
///
fn greetings(rep: usize) -> String {
    let mut greet = String::from("Hello");
    greet.push_str("o".repeat(rep).as_str());
    greet.push_str(" world!");
    greet
}



#[cfg(test)]
mod tests {
    use crate::greetings;

    #[test]
    fn greetings_works() {
        let g = greetings(4);
        assert_eq!(g, "Hellooooo world!");
    }
}



