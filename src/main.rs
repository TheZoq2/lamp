use std::path::PathBuf;
use std::sync::{Mutex, Arc};

mod led;
use led::LedCommand;
mod web;
mod button;

extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate sysfs_gpio;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use iron::headers;
use iron::modifiers::Header;
use std::thread;

// use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::sync::mpsc::{Sender};

pub fn handle_index_query(_: &mut Request) -> IronResult<Response>
{
    let header = Header(headers::ContentType::html());
    Ok(Response::with((status::Ok, header, PathBuf::from("files/index.html"))))
}



fn main() {
    let sender = Arc::new(Mutex::new(led::start_led_thread()));

    let mut led_router = router!
    (
        led: post "/" => web::LedQueryHandler::new(sender.clone())
    );
    thread::spawn(move || {
        //button::button_listener(14, sender)
    });

    let mut mount = mount::Mount::new();
    mount
        .mount("/", handle_index_query)
        .mount("static/", staticfile::Static::new(PathBuf::from("files")))
        .mount("led/", led_router);

    Iron::new(mount).http("192.168.1.8:3000").unwrap();

}
