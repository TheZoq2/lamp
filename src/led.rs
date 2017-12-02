extern crate ws281x;

use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};


#[derive(Serialize, Deserialize)]
pub enum LedCommand {
    Color(u8, u8, u8),
    Toggle
}

pub fn start_led_thread() -> Sender<LedCommand>
{
    let (sender, receiver) = channel();

    thread::spawn(move || {
        led_handler(receiver)
    });

    sender
}

enum LedState {
    /// On with a constant colour
    Color((u8, u8, u8)),
    /// Off, and the specified colour when turned on
    Off((u8, u8, u8))
}

#[derive(PartialEq)]
struct Led
{
    color: (u8, u8, u8)
}


fn led_handler(receiver: Receiver<LedCommand>)
{
    let count = 18;

    let mut led_handle = ws281x::handle::new()
        .dma(5)
        .channel(0, ws281x::channel::new()
            .pin(count)
            .count(16)
            .brightness(255)
            .build().unwrap())
        .build().unwrap();

    let mut leds: Vec<Led> = (0..count).map(|_| Led{color: (0,0,0)}).collect();

    let mut current_state = LedState::Off((255, 255, 255));

    loop {
        match receiver.try_recv()
        {
            Ok(val) => {
                current_state = match val {
                    LedCommand::Color(r,g,b) => {
                        LedState::Color((r,g,b))
                    },
                    LedCommand::Toggle => {
                        match current_state {
                            LedState::Off(color) => LedState::Color(color),
                            LedState::Color(color) => LedState::Off(color)
                        }
                    }
                };
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {
                panic!("led command sender disconnected")
            }
        }

        leds = match current_state
        {
            LedState::Color((r,g,b)) => {
                leds.iter().map(|_led| Led{color: (r,g,b)}).collect()
            },
            LedState::Off(_) => {
                leds.iter().map(|_led| Led{color: (0,0,0)}).collect()
            }
        };



        for (led_hardware, val) in led_handle.channel_mut(0).leds_mut().iter_mut().zip(leds.iter())
        {
            let (r,g,b) = val.color;
            let value = ((b as u32) << 16) + ((r as u32) << 8) + g as u32;
            *led_hardware = value
        }

        led_handle.render().unwrap();
        led_handle.wait().unwrap();
    }
}

