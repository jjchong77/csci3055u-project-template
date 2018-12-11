use std::io;
//code uses functions and std:io to perform basic math operations.
//uses std library
//::indicates a method or function is derived from whatever type precedes it.
fn main()
{

    loop
    {
        println!("What operation do you want to perform? (ADD:1, SUBTRACT:2, MULTIPLY:3, DIVIDE:4, EXIT:0)");

        let mut choice = String::new();//let sets up variable choice. Invokes String::new from std library.
                                       // mut sets a mutable variable
        io::stdin().read_line(&mut choice)//put input into mutable reference of choice
        .expect("Failed to read line");//similar to a catch statement
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //choice converts to an unsigned 32 bit number.
        //trim eliminates white space, and parse gets a number from the string

    	//could also write as std::io::stdin if you didn't import


        if choice == 1 // adding
            {
                println!("Addition selected.");
                println!("Enter first number.");
                let mut num1= String::new();
                io::stdin().read_line(&mut num1).expect("Failed to read line");
                let num1: f32 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter second number.");
                let mut num2= String::new();
                io::stdin().read_line(&mut num2).expect("Failed to read line");
                let num2: f32 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let answer:f32 = add(num1, num2);
                println!("{} + {} = {}", num1, num2, answer);
            }
        else if choice==2
            {
                println!("Subtraction selected.");
                println!("Enter first number.");
                let mut num1= String::new();
                io::stdin().read_line(&mut num1).expect("Failed to read line");
                let num1: f32 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter second number.");
                let mut num2= String::new();
                io::stdin().read_line(&mut num2).expect("Failed to read line");
                let num2: f32 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let answer:f32 = sub(num1, num2);

                println!("{} - {} = {}", num1, num2, answer);
            }
        else if choice==3
            {
                println!("Multiplication selected.");
                println!("Enter first number.");
                let mut num1= String::new();
                io::stdin().read_line(&mut num1).expect("Failed to read line");
                let num1: f32 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter second number.");
                let mut num2= String::new();
                io::stdin().read_line(&mut num2).expect("Failed to read line");
                let num2: f32 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let answer:f32 = mult(num1, num2);

                println!("{} * {} = {}", num1, num2, answer);
            }
        else if choice==4
            {
                println!("Division selected.");
                println!("Enter first number.");
                let mut num1= String::new();
                io::stdin().read_line(&mut num1).expect("Failed to read line");
                let num1: f32 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter second number.");
                let mut num2= String::new();
                io::stdin().read_line(&mut num2).expect("Failed to read line");
                let num2: f32 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let answer:f32 = div(num1, num2);
                println!("{} / {} = {}", num1, num2, answer);
            }
        else if choice ==0
            {
                println!("Exiting.");
                break;
            }
        else
            {
                println!("Cannot read input. Try again.");
            }
    }

}

fn add (x:f32, y:f32)->f32
{x+y}

fn sub (x:f32, y:f32)->f32
{x-y}

fn mult (x:f32, y:f32)->f32
{x*y}

fn div (x:f32, y:f32)->f32
{x/y}
