fn main() {
    //Without initial
    //let v: Vec<i32> = Vec::new();
    //With intial using vec! macro
    //let v = vec![1,2,3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
