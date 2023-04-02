#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::borrow::Cow;
use std::mem::{self};

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sofia_app_init() {
        assert_eq!(
            unsafe { sofia_app_check() },
            0
        );
    }
}
