fn main() {

    let number = 39;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    if number % 4 == 0{
        println!("number is divisible by 4");
    }else if number % 3 == 0{
        println!("number is divisible by 3");
    }else if number % 2 == 0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 2, 3, or 4");
    }

    //Using if in a let statement (because if is an expression)

    let condition : bool = true;
    
    let new_number = if condition{
        55
    }else{
        66
    };
    println!("The value of the number is {}", new_number);

    
}
