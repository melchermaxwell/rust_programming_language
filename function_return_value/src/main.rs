fn five() -> i32 {
    5
}

fn plus_one(x:i32) -> i32{
    x + 1
}

fn main() {
    let value: i32 = five();
    println!("The value of the function is {}", value);

    let valuex = plus_one(6);
    println!("The value of the second function is {}", valuex);
}
