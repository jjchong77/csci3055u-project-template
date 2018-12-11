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
