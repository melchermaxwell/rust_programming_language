mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn main() {
    //Order a breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change the order
    meal.toast = String::from("Wheat");
    //This next line should fail since seasonal_fruit isn't public
    //meal.seasonal_fruit = String::from("Apple");
}
