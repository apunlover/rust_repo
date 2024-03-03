use std::io;

fn is_version_greater_or_equal(version: &str, min_version: &str) -> bool {
    let version_number: i32 = version.trim_start_matches('A').parse().unwrap();
    let min_version_number: i32 = min_version.trim_start_matches('A').parse().unwrap();
    version_number >= min_version_number
}

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
