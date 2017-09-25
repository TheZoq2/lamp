extern crate serde_json;

use std::sync::mpsc::{Sender};
use std::io::Read;

use led::LedCommand;

use iron::prelude::*;
use iron::Handler;
use iron::status;

use std::sync::{Mutex, Arc};

pub struct LedQueryHandler
{
    command_sender: Arc<Mutex<Sender<LedCommand>>>
}

impl LedQueryHandler
{
    pub fn new(command_sender: Arc<Mutex<Sender<LedCommand>>>) -> LedQueryHandler
    {
        LedQueryHandler {
            command_sender
        }
    }
}

impl Handler for LedQueryHandler
{
    fn handle(&self, request: &mut Request) -> IronResult<Response>
    {
        let mut body_content = String::new();
        request.body.read_to_string(&mut body_content);

        // Attempt to deserialize the command
        match serde_json::from_str(&body_content)
        {
            Ok(val) => {
                let command_sender = self.command_sender.lock().unwrap();
                command_sender.send(val).expect("Receiver disconnected");
                Ok(Response::with((status::Ok, "\"Led handled\"")))
            },
            Err(e) => Ok(Response::with((status::NotFound, format!("Not a valid LedCommand {}", e))))
        }
    }
}


