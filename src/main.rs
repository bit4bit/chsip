#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::sync::{Arc};
use std::sync::Mutex;

struct Application {
    internal: Mutex<*mut sofia_app_t>
}
unsafe impl Send for Application {}


impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            let mut app = *self.internal.lock().unwrap();
            sofia_app_deinit(app);
            sofia_app_destroy(&mut app);
        }
    }
}

impl Application {
    fn new(host: &str, port: i32) -> Self {
        unsafe {
            let app = sofia_app_create();
            let details = sofia_app_details_create(app);
            sofia_app_details_set_bindhost(details, CString::new(host).unwrap().as_ptr(), port);
            let _ = sofia_app_init(app, details) || panic!("fails to sofia_app_init");
            
            Self { internal: Mutex::new(app) }
        }
    }

    fn iterate(&mut self, interval: i64) {
        unsafe {
            let app = *self.internal.lock().unwrap();
            sofia_app_iterate(app, interval);
        }
    }
        
}

#[tokio::main]
async fn main() {
    //i don't know how, but sofia knows
    //if the iterations runs in another thread
    let mut app = Application::new("localhost", 5070);
    
    loop {
        //blocks thread
        app.iterate(10);
    }
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
    fn it_application_init() {
        Application::new("localhost", 5070);
    }

    #[test]
    fn it_application_iterate() {
        unsafe {
            let mut app = Application::new("localhost", 5071);
            app.iterate(100);
        }
    }
}
