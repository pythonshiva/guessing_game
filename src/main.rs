extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1,101);
    // println!("The secret number is: {}", secret_num);
	loop {
		println!("Input your guess");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Something has gone wrong");
	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => 	{
	    		println!("Enter a numaric value");
	    		continue;
		    	}
	    };
	    println!("you have entered: {}", guess);

	    match guess.cmp(&secret_num) {
	    	Ordering::Less => println!("Too small"),
	    	Ordering::Greater => println!("Too big"),
	    	Ordering::Equal => {
	    		println!("Congrats, You have guessed correct number!");
	    		break;
	    	}
	    }
	}
    
}
