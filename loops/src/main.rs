fn main() {
    forloopwithrange();
}

fn basicloop() {
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Counter final value is {}", result);
}
fn whileloop(){
    let mut number = 4;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("Liftoff!")
}
fn loopthroughcollection(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index +1;
    }
}
fn betterloopthroughcollection(){
    let a = [10,20,30,40,50];
    for element in a.iter(){
        println!("the value is: {}", element);
    }
}
fn forloopwithrange(){
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!")
}
