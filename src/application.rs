include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CString, CStr};
use std::sync::Mutex;

use log::{info};

#[derive(Debug)]
pub struct SofiaAppTag<'a>{
    ns: &'a str,
    name: &'a str,
    value: &'a str
}

pub type SofiaAppTags<'a> = Vec<SofiaAppTag<'a>>;

pub trait ApplicationBehavior {
    fn handle_incoming(&mut self, event: i32, event_name: &str, status: i32, phrase: &str, tags: SofiaAppTags);
}

#[repr(C)]
pub struct Application<'a> {
    internal: *mut sofia_app_t,
    behavior: &'a mut dyn ApplicationBehavior
}
unsafe impl Send for Application<'_> {}


impl Drop for Application<'_> {
    fn drop(&mut self) {
        unsafe {
            let mut app = self.internal;
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

impl<'a> Application<'a> {
    pub fn new(behavior: &'a mut dyn ApplicationBehavior) -> Self {
        unsafe {
            let app = sofia_app_create();
            Self { internal: app, behavior: behavior }
        }
    }

    pub fn init(&mut self, host: &str, port: i32) {
        let app = self.internal;

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

    pub fn iterate(&mut self, interval: i64) {
        unsafe {
            let app = self.internal;
            sofia_app_iterate(app, interval);
        }
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
        fn new() -> Self {
            Self{_last_event: "".to_string()}
        }

        fn last_event(&self) -> String {
            self._last_event.clone()
        }
    }
    impl ApplicationBehavior for ApplicationBehaviorChecker {
        fn handle_incoming(&mut self, event: i32, event_name: &str, status: i32, phrase: &str, tags: SofiaAppTags) {
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
        let mut behavior = ApplicationBehaviorChecker::new();
        Application::new(&mut behavior).init("localhost", 5070);
    }

    #[test]
    fn it_application_iterate() {
        let mut behavior = ApplicationBehaviorChecker::new();
        let mut app = Application::new(&mut behavior);
        app.init("localhost", 5071);
        app.iterate(100);
    }

    #[test]
    fn it_application_behavior_receive_event() {
        let mut behavior = ApplicationBehaviorChecker::new();
        let mut app = Application::new(&mut behavior);
        app.init("localhost", 5072);
        app.iterate(100);

        drop(app);

        let last_event = behavior.last_event();
        assert_eq!(
            last_event,
            "nua_r_shutdown"
        );
    }

}
