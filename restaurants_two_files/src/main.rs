mod front_of_house;  //Declaring front_of_house module whose body wull be in src/front_of_house

pub use crate::front_of_house::hosting;

pub fn eat_at_resaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {
    println!("Hello, world!");
}
