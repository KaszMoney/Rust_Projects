use std::io;
//Simply acessing input and output capabilities from the Standard Library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    //println! is a macro to print something to the terminal

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {
        let mut guess = String::new();
        //let is used to define a variable
        //mut means the variable is mutable, and can be changed later on
        //In rust, all variables are immutable by default


        //The String::new() part is where an instance of a new string is returned
        //String is a string type provided by the standard library
        //::new means that we are making a new, empty string

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        //We already included input and output earlier, so now we can use a funciton from it, such as stdin, which is standard input

        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
        };

        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too little!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }

}
