extern crate memchr;
use memchr::memchr;
use memchr::memchr2;
fn main()
{
    let data=b"about to run";

    let result=memchr(b'b', data);
    println!("{:?}",result);
    assert_eq!(memchr(b'b', data), Some(1));
    let data2=b"let's try this again";
    let result2=memchr(b's', data2);
    println!("second: {:?}",result2);
}
