fn main() {
    clone_test();
}
fn shallow_copy(){
    let s1 = String::from("hello");
    let s2 = s1;
    
    // println!("{}, world!", s1);
}
fn clone_test(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
