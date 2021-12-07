// impl Message {
//     fn call(&self) {
//         //method body
//     }
// }
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Texas
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    let token = Coin::Dime;
    println!("The value of a Dime is {}", value_in_cents(token));
    let state = UsState::Texas;
    let tokenq = Coin::Quarter(state);
    println!("The value of a Quarter is {}", value_in_cents(tokenq));
    println!("{}",value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
