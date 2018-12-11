fn main ()
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
