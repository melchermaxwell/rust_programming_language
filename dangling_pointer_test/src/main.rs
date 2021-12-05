fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} //here s goes out of scope and is dropped.  Its memory goes away
//would work if we removed the & reference and returned the string directly