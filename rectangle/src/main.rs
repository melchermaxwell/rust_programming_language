#[derive(Debug)] //allows us print struct
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // refactoring with tuples
    //let rectangle = (30,50);
    //refactoring with struct
    let rect1 = Rectangle {
        height: 30,
        width: 30,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) //We want to borrow the struct rather than take ownership.
        //This way main retains its ownership and can continue using rect1
    );
    //:?
    println!("rect1 is {:?}", rect1);
    //:#?
    println!("rect1 is {:#?}", rect1);

    //Using methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// fn area(dimensions:(u32,u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
fn area(rect:&Rectangle) -> u32 {
    rect.height * rect.width
}
