use std::io;
fn main()
{
    //code will take user entry and enter it all into one vector, before adding all entries in the vector into an empty string and printing it out.
    let mut v: Vec<char> = Vec::new();
    let mut exit:bool =false;
    let mut text=String::new();
    let mut testchar:char='y';
    loop
    {
        println!("Enter text. If you wish to exit, enter ~.");
        let mut entry=String::new();//sets entry variable. Uses cat for default.
        io::stdin().read_line(&mut entry).expect("Failed to read line");//takes use input and puts it into mutable reference of entry
        for c in entry.chars()
        {

            if c=='~'
             {
                 println!("found ~");
                exit=true;
                 break;
             }
             else
            {
                v.push(c);
            }
        }
        if (exit)
        {
            break;
        }
    }
    println!("Building text.");
    for i in &v
    {
    text.push(*i);
    }
    println!("{}", text);
}
