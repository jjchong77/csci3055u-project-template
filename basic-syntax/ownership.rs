struct foo {
    data: u32,
}

fn main() {
    let st = foo { data: 5 };

    // pass reference to function
    print_ref(&st);

    // You can continue to call the function
    print_ref(&st);

    //invoke print val using variable ST instead of reference
    print_val(st);

    print_val(st);
    // get a compiler error, since st was "used up" by the first print_val; it was given to the printval method
    // which closed it's memory after finishing
}


fn print_ref(st: &Something) {//uses reference
    println!("foo: {}", st.value);
}

fn print_val(st: Something) {//uses variable/value
    println!("foo: {}", st.value);
}
