use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your guess");

    let mut guess = String::new();
	let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Something has gone wrong");
    println!("you have entered: {}", guess);
}
