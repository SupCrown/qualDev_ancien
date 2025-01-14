

fn main() {
    let repetition = 5;
    let res = greetings(repetition);
    println!("{}",res);
}

fn greetings(rep: usize)-> String {
    let mut greet = String::from("Hello");
    greet.push_str("o".repeat(rep).as_str());
    greet.push_str(" world!");
    greet
}

