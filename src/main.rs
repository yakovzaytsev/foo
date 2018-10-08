#![feature(tool_lints)]

// use std::cast;
// use std::fmt;

#![allow(dead_code)]
#![warn(clippy::empty_enum)]
enum Foo {
    // empty enum
}

// impl fmt::Display for Foo {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "a Foo")
//     }
// }

fn main() {
    let vec: Vec<i32> = Vec::new();

    if vec.len() <= 0 {
        println!("Hello, world!");
    }
}
