fn main() {

    let s1 = String::from("Hello");

    //let (s2, len) = calculate_length(s1);

    let lentwo = calculate_length(&s1);

    println!("the length of '{}' is {}", s1, lentwo);
}

// fn calculate_length(s: String)->(String, usize){
//     let length = s.len();

//     (s,length)
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}
