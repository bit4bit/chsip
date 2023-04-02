#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::borrow::Cow;
use std::mem::{self};
use std::ffi::{CStr, CString};

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_su_init() {
        unsafe {
            su_init();
            su_deinit();
        }
    }

    #[test]
    fn it_su_home() {
        unsafe {
            let mut home: su_home_t = mem::zeroed();
            su_home_init(&mut home);
            su_home_deinit(&mut home);
        }
    }

    #[test]
    fn it_su_strdup() {
        unsafe {
            let mut home: su_home_t = mem::zeroed();
            su_home_init(&mut home);

            let dup = su_strdup(&mut home, CString::new("hola").expect("cstring").as_ptr());

            //CString::from_raw(dup).to_str() drops dup
            assert_eq!(
                CStr::from_ptr(dup).to_string_lossy(),
                //or
                //String::from_utf8_lossy(CStr::from_ptr(dup).to_bytes()).to_string(),

                Cow::Borrowed("hola")
            );

            su_home_deinit(&mut home);
        }
    }
}
