extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//uses std library
//::indicates a method or function is derived from whatever type precedes it.
fn main()
{

    println!("Guess the number!");
    let mut lives:i16=10;
    let answer = rand::thread_rng().gen_range(1, 101);
//    println!("DEBUG: Answer {}", secret_number);

    loop
    {
        println!("Enter your guess. You have {} lives remaining.", lives);

        let mut guess = String::new();//let sets a variable. let mut sets a mutable variable, can be changed
                                        //new is a function of String
        io::stdin().read_line(&mut guess)//takes use input and puts it into mutable reference of guess
        .expect("Failed to read line");//similar to a catch statement
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //guess converts to an unsigned 32 bit number. guess is the original value
        //trim eliminates white space, and parse gets a number from the strnig

    	//could also write as std::io::stdin if you didn't import
        println!("You guessed: {}", guess); //{} is a place holder for guess
        match guess.cmp(&answer)//match made of "arms"; for a given value it'll do the code.
                                      //cmp is an enum, returns either less, greater, or equal
        {
          Ordering::Less => {   lives=lives-1;
                                println!("Too small!")
                            },
          Ordering::Greater =>
                            {
                                lives=lives-1;
                                println!("Too big!")
                            },
          Ordering::Equal => {println!("You win!"); break;}
        }
        if lives==0
           {
               println!("Out of lives!");
               break;
           }

    }
}
