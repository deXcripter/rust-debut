// import the enums.rs file
mod classes;
mod enums;
use classes::manage_user;
use enums::{enums, enums_with_data};

fn main() {
    enums();
    enums_with_data();
    manage_user();
}

// Arithmetic overflows and underflows
