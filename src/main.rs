#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::ffi::c_void;
use std::sync::{Arc};
use std::sync::Mutex;
use libc;

use log::{info, trace, warn};
use simple_logger::SimpleLogger;

trait ApplicationBehavior {
    fn handle_incoming(&mut self, event: i32, status: i32, phrase: &str);
}

pub struct Application {
    internal: Mutex<*mut sofia_app_t>,
    behavior: Box<dyn ApplicationBehavior>
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

pub type sofia_app_handle_incoming_cb = ::std::option::Option<
    unsafe extern "C" fn(
        event: ::std::os::raw::c_int,
        status: ::std::os::raw::c_int,
        phrase: *const ::std::os::raw::c_char,
        user_data: *mut Application
    ),
>;

extern "C" {
    pub fn sofia_app_init(
        app: *mut sofia_app_t,
        bindhost: *const ::std::os::raw::c_char,
        bindport: ::std::os::raw::c_int,
        handle_incoming: sofia_app_handle_incoming_cb,
        handle_incoming_user_data: *mut Application
    ) -> bool;
}

unsafe extern "C" fn dispatch_handle_incoming(event: i32, status: i32, phrase: *const i8, app: *mut Application) {
    (*app).handle_incoming(event, status, "hola");
}

impl Application {
    fn new(behavior: Box<dyn ApplicationBehavior>) -> Self {
        unsafe {
            let app = sofia_app_create();
            Self { internal: Mutex::new(app), behavior: behavior }
        }
    }

    fn init(&mut self, host: &str, port: i32) {
        let app = *self.internal.lock().unwrap();

        unsafe {
            let host = CString::new(host).unwrap();
            let mut obj = Box::new(self);
            let _ = sofia_app_init(app,
                                   host.as_ptr(),
                                   port,
                                   Some(dispatch_handle_incoming),
                                   *obj) || panic!("fails to sofia_app_init");
        }
    }

    fn handle_incoming(&mut self, event: i32, status: i32, phrase: &str) {
        self.behavior.handle_incoming(event, status, phrase);
    }

    fn iterate(&mut self, interval: i64) {
        unsafe {
            let app = *self.internal.lock().unwrap();
            sofia_app_iterate(app, interval);
        }
    }
        
}


struct ApplicationBehaviorDumb {
}

impl ApplicationBehaviorDumb {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl ApplicationBehavior for ApplicationBehaviorDumb {
    fn handle_incoming(&mut self, event: i32, status: i32, phrase: &str) {
        eprintln!("dumb handle incoming");
    }
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    info!("starting..");
    //i don't know how, but sofia knows
    //if the iterations runs in another thread
    let behavior = ApplicationBehaviorDumb::new();
    let mut app = Application::new(behavior);
    app.init("localhost", 5070);
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
        let behavior = ApplicationBehaviorDumb::new();
        Application::new(behavior).init("localhost", 5070);
    }

    #[test]
    fn it_application_iterate() {
        let behavior = ApplicationBehaviorDumb::new();
        let mut app = Application::new(behavior);
        app.init("localhost", 5071);
        app.iterate(100);
    }

}
