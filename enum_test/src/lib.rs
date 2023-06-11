mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist")
        }
        pub fn seat_at_table() {
            println!("seat_at_table")
        }
    }
}

pub fn host_add_to_waitlist(){
    front_of_house::hosting::add_to_waitlist();
}

pub fn host_seat_at_table(){
    front_of_house::hosting::seat_at_table();
}