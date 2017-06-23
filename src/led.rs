use std::time::Duration;

extern crate ws281x;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};


#[derive(Serialize, Deserialize)]
pub enum LedCommand {
    Color(u8, u8, u8)
}

pub fn start_led_thread() -> Sender<LedCommand>
{
    let (sender, receiver) = channel();

    thread::spawn(move || {
        led_handler(receiver)
    });

    sender
}


fn led_handler(receiver: Receiver<LedCommand>)
{
    let mut led_handle = ws281x::handle::new()
        .dma(5)
        .channel(0, ws281x::channel::new()
            .pin(18)
            .count(3)
            .brightness(255)
            .build().unwrap())
        .build().unwrap();

    let mut current_command = LedCommand::Color(0, 0, 0);

    loop {
        match receiver.try_recv()
        {
            Ok(val) => {
                current_command = val
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {
                panic!("led command sender disconnected")
            }
        }

        match current_command
        {
            LedCommand::Color(r,g,b) => {
                let value = ((b as u32) << 16) + ((r as u32) << 8) + g as u32;

                for led in led_handle.channel_mut(0).leds_mut().iter_mut() {
                    *led = value
                }
            }
        }

        led_handle.render().unwrap();
        led_handle.wait().unwrap();

        thread::sleep(Duration::from_millis(500));
    }
}

