mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

fn main() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
