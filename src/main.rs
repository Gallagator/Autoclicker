use std::{
    sync::Arc,
    sync::Mutex,
};

mod clicker;
use clicker::Clicker;

mod event;
use event::kbd_event::*;

pub use input_event_codes::*;

/* Frequency in Hz*/
const FREQ_MEAN: f64 = 30f64;
const PERIOD_SD_PROPORTION: f64 = 0.4f64;
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
                    KBD_PRESSED);
    event.add_event(KEY_ESC, 
                    move || {
                        let mut d = done_clone.lock().unwrap();
                        *d = true;
                    }, 
                    KBD_PRESSED);
    
    let clicker = Clicker::new(FREQ_MEAN, PERIOD_SD_PROPORTION, MAX_SDS);
    event.start().unwrap();
    while !(*done.lock().unwrap()){
        if *is_clicking.lock().unwrap() { clicker.sleep_and_click(); }
        else { clicker.sleep()};
    }
    event.stop();
}
