# _Report on Rust_

- _Jeremy Chong_
- _jeremy.chong@uoit.net_

## About the language

> _Describe the language_
>
> - History
> - Ownership feature helps manage memory.
> - comes with cargo, which manages projects and imports.
> - 

## About the syntax

> _give some code snippet of the language_
    
*declaring and shadowing variables*
```
//basic variables
let mut num = 0; //make mutable variable
let num2=0; //non mutable variable. num2=num2+1; Would not work, because num2 is not muteable.
let mysignedint:i32= 1;
let myfloat:f32=1.0;
let my_unsigned_scalar:u64=1;
let mychar='x'; //uses inference to set type as char
let num2=num2+1; //called shadowing a variable; allows you to change a normally unmuteable variable. This gives you more control over what can be done.

let myarr = [1, 2, 3, 4, 5]; //Array - all elements must have same type, and arrays have fixed lengths.
let num3 = myarr[0];
let num4 = myarr[1];

let mytup = (300, 3.14, 0);
let three_hundred = mytup.0; //access tuple element directly.
println!("{}", three_hundred);
let (x, y, z) = mytup; //tuple.  this line copies the contents of the tuple, and turns it into indvidual variables.
println!("The value of y is: {}", y);//prints out 3.14.
println!("{}", mytup.2); //also prints out 3.14
```
*if, else, and if else*

```
fn main ()
{
    if num<0.0
    {
        println!("negative");
    }
    else if num > 0.0
    {
        println!("positive");
    }
    else
    {
        println!("zero");
    }
}

```

*loops*
```fn main ()
{
   let mut num=0;
    loop
    {
        num=num+1;
        println!("Iteration no {}", num);
        if (num==10)
        {break;}
    }
    let myar = [1, 2, 3, 4, 5];

    for x in 0..10 //for loop
    {
    println!("Iteration no {}", x); // x: i32
    }
    println!("Second for loop");
    for x in 0..myar.len()
    {
        println!("Iteration no {}", myar[x]); // prints the elements in the array

    }
}
```
*defining functions*
```
fn main() {
    println!("Main function!");
    let num = 32.0;
    let num2=3.0;
    let num3:f32=ret();
    fn2(num, num2);
    println!("{}", num3);
}

fn fn2(x:f32, y:f32) { //takes float variable x and y
    println!("{} {}", x, y);
}

fn ret()->f32 //->f32 means the function returns a float
{
let mut num=4.0;
    num+4.0
}

}

```
## About the tools

> Rustup, the installer, automatically installs a package manager known as Cargo. Similar to Lein, Cargo can automatically create projects and can build them from filse located in the src folder. It does require you to have the Visual Basic Studio tools for C installed. 

>Projects can be created through Cargo with the command ```cargo new project_name```. It will create several directories and a file that holds information on the project, the ```Cargo.toml``` file. Here is an example. It holds information on the name, version, author, and edition of the projects, and more importantly other libraries that it might rely on.

```[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

>Cargo also has the ```build``` ```check``` and ```run``` commands. Each of these is simply initiated by typing ```cargo build```, ```cargo check```, or ```cargo run```.

>Cargo Build will compile the code as an executable, then place it in the target/debug/project-name.exe path. It can then be run through command line.

>Cargo Check scans the code to see if it can compile, but does not actually build it. It is useful for making changes quickly without waiting while the code compiles.

>Cargo Run builds the code and then executes it in the command line.

## About the standard library

> _Give some examples of the functions and data structures
> offered by the standard library_.
>String, as well as IO are inside the std library.

## About open source library

> _Describe at least one contribution by the open source
community written in the language._
The open source library function for Rust is located on https://crates.io/. For the purpose of this assignmen, I opted to use the 
# Analysis of the language

> _
Rust is a language that has some support for functional programming.
It allows the user to utilize macros for meta programming - writing code that writes more code.

3.	Symbol resolution and its support for closure
4.	Scoping rules supported by the language: lexical vs dynamic scoping
5.	Functional programming constructs either as part of the language or supported by the standard library of the runtime.
6.    Rust is a statically typed system. That is to say - it must know the types of all variables it uses at the time of compilation. HOWEVER, it also supports type inference. This means that even if you don't declare the type, it will assign a type based off the assigned values.
7.	Strengths and weaknesses of the language
Rust's main focus is on being memory safe - it avoids memory leaks and such. It has an "ownership" feature which allows it to make memory safe guarantees without needing a garbage collector. 




