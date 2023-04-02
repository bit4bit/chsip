#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sofia_app_check() {
        assert_eq!(
            unsafe { sofia_app_check() },
            0
        );
    }

    #[test]
    fn it_sofia_app_init() {
        unsafe {
            let mut app = sofia_app_create();
            let details = sofia_app_details_create(app);
            sofia_app_details_set_bindhost(details, CString::new("localhost").unwrap().as_ptr(), 5070);
            assert_eq!(
                sofia_app_init(app, details),
                true
            );

            sofia_app_deinit(app);
            sofia_app_destroy(&mut app);
        }
    }

    #[test]
    fn it_sofia_app_iterate() {
        unsafe {
            let mut app = sofia_app_create();
            let details = sofia_app_details_create(app);
            sofia_app_init(app, details);

            sofia_app_iterate(app, 1000);

            sofia_app_deinit(app);
            sofia_app_destroy(&mut app);
        }
    }
}
