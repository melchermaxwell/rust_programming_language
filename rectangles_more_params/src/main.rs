#[derive(Debug)] //allows us print struct
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, recty:&Rectangle) -> bool{
        self.height >= recty.height && self.width >= recty.width
    }
}


fn main() {
   let rect1 = Rectangle{ height:30, width:30 };
   let rect2 = Rectangle{ height:90, width:30 };
   let rect3 = Rectangle{ height:30, width:40 };

    println!("Check can {:?} hold {:?} is: {}", rect2, rect1, rect2.can_hold(&rect1));
}
