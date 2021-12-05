fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    //Can't use s now that value was moved
    //println!("{}",s);

    let x = 5;

    makes_copy(x);

    //we can still use x because i32 works as copy
    println!("{}",x)
}

fn takes_ownership(some_string: String){
    println!("This is the string {}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer)
}
