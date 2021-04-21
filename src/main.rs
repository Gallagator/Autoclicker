use std::{
    sync::{ Arc, Mutex, },
};

mod clicker;
use clicker::Clicker;

mod event;
use event::kbd_event::{KbdEvent, KEY_PRESSED, KEY_REPEATED, KEY_RELEASED,};

pub use input_event_codes::*;
/* Frequency in Hz*/
const FREQ_MEAN: f64 = 20f64;
const PERIOD_SD_PROPORTION: f64 = 0.1f64;
const MAX_SDS: f64 = 3f64;

fn main() {
    let done = Arc::new(Mutex::new(false));
    let done_clone = done.clone();
    let is_clicking = Arc::new(Mutex::new(false));
    let is_clicking_clone = is_clicking.clone();
    let mut event = KbdEvent::new();
    event.add_event(KEY_R, 
                    move || {
                        let mut clicking = is_clicking_clone.lock().unwrap();
                        *clicking = !*clicking;
                    }, 
                    KEY_PRESSED);
    event.add_event(KEY_ESC, 
                    move || {
                        let mut d = done_clone.lock().unwrap();
                        *d = true;
                    }, 
                    KEY_PRESSED);
    
    let clicker = Clicker::new(FREQ_MEAN, PERIOD_SD_PROPORTION, MAX_SDS);
    event.start().unwrap();
    while !(*done.lock().unwrap()){
        if *is_clicking.lock().unwrap() { clicker.sleep_and_click(); }
        else { clicker.sleep()};
    }
    event.stop();
}
