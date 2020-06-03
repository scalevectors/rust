use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    
    //The rand::thread_rng() function will give us the particular random number generator 
    // that we’re going to use: one that is local to the current thread of execution 
    //and seeded by the operating system. 
    // Then we call the gen_range() method on the random number generator.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Enter number: ");
        let mut guess = String::new();
           
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => {
                    println!("Ok({})", num);
                }
                Err(_) => {
                    println!("Please enter number");
                    continue;
                }
            };            

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }

        println!("Your guess: {}", guess);
    }
}
