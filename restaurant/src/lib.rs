mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use front_of_house::serving;
use front_of_house::serving as s;

fn call_serving() {
    front_of_house::serving::take_order();
    serving::take_order();
    s::take_order();
}
