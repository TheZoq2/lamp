use std::path::PathBuf;
use std::sync::Mutex;

mod led;
use led::LedCommand;
mod web;

extern crate iron;
extern crate mount;
extern crate staticfile;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use iron::headers;
use iron::modifiers::Header;

// use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::sync::mpsc::{Sender};

pub fn handle_index_query(_: &mut Request) -> IronResult<Response>
{
    let header = Header(headers::ContentType::html());
    Ok(Response::with((status::Ok, header, PathBuf::from("files/index.html"))))
}



fn main() {
    let sender = led::start_led_thread();

    let mut led_router = router!
    (
        led: post "/" => web::LedQueryHandler::new(Mutex::new(sender))
    );

    let mut mount = mount::Mount::new();
    mount
        .mount("/", handle_index_query)
        .mount("static/", staticfile::Static::new(PathBuf::from("files")))
        .mount("led/", led_router);

    Iron::new(mount).http("192.168.0.104:3000").unwrap();

}
