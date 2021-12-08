mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        mod back_of_house {
            fn fix_incorrect_order(){
                cook_order();
                super::serve_order();
                //Super goes to parent module; in this case, serving
            }
            fn cook_order(){}
        }
    }
}

pub fn eat_at_resaurant(){
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}