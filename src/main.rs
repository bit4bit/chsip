mod application;

use log::{info};
use simple_logger::SimpleLogger;
use crate::application::{Application, ApplicationBehavior, SofiaAppTags};

pub struct ApplicationBehaviorDumb {
}

impl ApplicationBehaviorDumb {
    pub fn new() -> Self {
        Self {}
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
    let mut behavior = ApplicationBehaviorDumb::new();
    let mut app = Application::new(&mut behavior);
    app.init("localhost", 5070);
    loop {
        //blocks thread
        app.iterate(10);
    }
}
