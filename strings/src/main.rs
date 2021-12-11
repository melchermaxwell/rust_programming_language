fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s3 = String::from("Hello ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; //note s3 has been moved here and can no longer be used

    let s6 = String::from("Hello ");
    let s7 = String::from("world!");
    let s8 = String::from("Hello ");
    let s9 = format!("{}-{}-{}", s6, s7, s8);
}
