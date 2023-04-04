#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CString, CStr};
use std::sync::Mutex;
use std::any::Any;

use log::{info};
use simple_logger::SimpleLogger;

#[derive(Debug)]
struct SofiaAppTag<'a>{
    ns: &'a str,
    name: &'a str,
    value: &'a str
}

type SofiaAppTags<'a> = Vec<SofiaAppTag<'a>>;

trait ApplicationBehavior {
    fn handle_incoming(&mut self, event: i32, event_name: &str, status: i32, phrase: &str, tags: SofiaAppTags);
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
        event_name: *const ::std::os::raw::c_char,
        status: ::std::os::raw::c_int,
        phrase: *const ::std::os::raw::c_char,
        tags: *mut sofia_app_tag_t,
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

unsafe extern "C" fn dispatch_handle_incoming(
    event: i32,
    event_name: *const i8,
    status: i32,
    phrase: *const i8,
    mut tags: *mut sofia_app_tag_t,
    app: *mut Application) {
    let phrase = CStr::from_ptr(phrase).to_str().unwrap();
    let event_name = CStr::from_ptr(event_name).to_str().unwrap();
    let mut rtags: SofiaAppTags = SofiaAppTags::new();
    loop {
        if !tags.is_null() {
            let ns = CStr::from_ptr((*tags).ns).to_str().expect("fails to get tag ns");
            let name = CStr::from_ptr((*tags).name).to_str().expect("fails to get tag name");
            let value = CStr::from_ptr((*tags).value).to_str().expect("fails to get tag value");

            if name != "tag_null" {
                rtags.push(SofiaAppTag{ns: ns, name: name, value: value});
            }
            tags = (*tags).next;
        } else {
            break;
        }
    }

    (*app).handle_incoming(event, event_name, status, phrase, rtags);
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
            let obj = Box::new(self);
            let _ = sofia_app_init(app,
                                   host.as_ptr(),
                                   port,
                                   Some(dispatch_handle_incoming),
                                   *obj) || panic!("fails to sofia_app_init");
        }
    }

    fn handle_incoming(&mut self, event: i32, event_name: &str, status: i32, phrase: &str, tags: SofiaAppTags) {
        self.behavior.handle_incoming(event, event_name, status, phrase, tags);
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
    fn handle_incoming(&mut self, event: i32, event_name: &str, status: i32, phrase: &str, tags: SofiaAppTags) {
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

    #[derive(Clone)]
    struct ApplicationBehaviorChecker {
        _last_event: String
    }
    impl ApplicationBehaviorChecker {
        fn new() -> Box<Self> {
            Box::new(Self{_last_event: "".to_string()})
        }

        fn last_event(&self) -> &String {
            &self._last_event
        }
    }
    impl ApplicationBehavior for ApplicationBehaviorChecker {
        fn handle_incoming(&mut self, event: i32, event_name: &str, status: i32, phrase: &str, tags: SofiaAppTags) {
            eprintln!("CHECKER {}", event_name);
         self._last_event = event_name.to_string();
        }
    }

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

    #[test]
    fn it_application_behavior_receive_event() {
        let mut behavior = ApplicationBehaviorChecker::new();
        let mut app = Application::new(behavior);
        app.init("localhost", 5072);
        app.iterate(100);
        drop(app);


        assert_eq!(
            "nua_r_shutdown",
            "nua_r_shutdown"
        );
    }

}
