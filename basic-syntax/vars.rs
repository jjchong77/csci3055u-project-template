fn main ()
{
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

}
