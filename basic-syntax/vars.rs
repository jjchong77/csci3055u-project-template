fn main ()
{
    let mut num = 0; //make mutable variable
    let num2=0;
    println!{"{}", num2};
    //num2=num2+1; would not work, because num2 is not muteable.
    let num2=num2+1; //called shadowing a variable; allows you to change a normally unmuteable variable. This gives you more control over what can be done.
    println!{"{}", num2};
}
