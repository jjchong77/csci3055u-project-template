# _Report on Rust_

- _Jeremy Chong_
- _jeremy.chong@uoit.net_

## About the language

> _Describe the language_
>
> - History
> - Some interesting features

## About the syntax

> _give some code snippet of the language_

*declaring and shadowing variables*
```
let mut num = 0; //make mutable variable
let num2=0;
//num2=num2+1; would not work, because num2 is not muteable.
let num2=num2+1; //called shadowing a variable; allows you to change a normally unmuteable variable. This gives you more control over what can be done.
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
```
 let mut num=0;
    loop
    {
        num=num+1;
        println!("Iteration no {}", num);
        if (num==10)
        {break;}
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

> Rustup, the installer, automatically installs a package manager known as Cargo. Similar to Lein, Cargo can automatically create projects and run them either from a file located in the src folder, or from the directory through the command ```cargo run```. It does require you to have the Visual Basic Studio tools for C installed. 

>Projects can be created through Cargo with the command ```cargo new project_name```. It will create several directories and a file that holds information on the project, the ```Cargo.toml``` file. Here is an example.

```[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

>Cargo also has the ```build``` 

## About the standard library

> _Give some examples of the functions and data structures
> offered by the standard library_.

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
Rust's main focus is on being memory safe - it avoids memory leaks and such.



