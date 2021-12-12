mod a;
use a::{
    test_3
};

pub fn test_1() {
    println!("Test function 1 called");
}

fn test_2() {
    println!("Test function 2 called");
}

pub fn test_4() {
    test_2();
    test_3();
}