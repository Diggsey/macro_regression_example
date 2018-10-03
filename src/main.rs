#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

#[derive(DbEnum, Debug)]
pub enum TestEnum {
    A,
    B
}

fn main() {
    println!("Hello, world!");
}
