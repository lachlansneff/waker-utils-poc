use std::any::type_name;
use waker_util::Waker;

pub fn print_waker_type() {
    println!("{}", type_name::<Waker>());
}
