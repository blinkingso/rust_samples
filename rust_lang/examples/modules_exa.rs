/// managing growing projects
fn main() {}

mod front_of_house {
    pub(crate) mod hosting {
        pub(crate) fn add_to_wait_list() {
            println!("add to wait list");
        }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub(crate) use crate::front_of_house::hosting;

pub mod prelude {
    pub use crate::front_of_house::*;
}

#[cfg(test)]
mod test {
    use super::hosting;
    use crate::front_of_house::hosting::add_to_wait_list;
    use crate::prelude::hosting::add_to_wait_list as prelude_wait;

    #[test]
    fn test_package() {
        add_to_wait_list();
        hosting::add_to_wait_list();
        prelude_wait();
    }
}
