#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::mem::{self};
use std::ffi::CString;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod su_tests {
    use super::*;

    #[test]
    fn it_su_init() {
        unsafe {
            su_init();
            su_deinit();
        }
    }

    #[test]
    fn it_su_strdup() {
        unsafe {
            let mut home: su_home_t = mem::zeroed();
            su_home_init(&mut home);

            let dup = su_strdup(&mut home, CString::new("hola").expect("cstring").as_ptr());
            assert_eq!(
                CString::from_raw(dup).to_str(),
                Ok("hola")
            );

            //why double free?
            //rust drop?
            //su_home_deinit(&mut home);
        }
    }
}
