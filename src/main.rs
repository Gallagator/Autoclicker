mod clicker;
use clicker::clicker::Clicker;

mod input;
use input::kbd_input::kbd_in;
pub use input_event_codes::*;
/* Frequency in Hz*/
const FREQ_MEAN: f64 = 100f64;
const PERIOD_SD_PROPORTION: f64 = 0.1f64;
const MAX_SDS: f64 = 3f64;

fn main() {
    //let clicker = Clicker::new(FREQ_MEAN, PERIOD_SD_PROPORTION, MAX_SDS);
    //let mut i = 0;
    //while i < 10000 {
    //    clicker.sleep_and_click();
    //    i += 1;
    //}
    let mut kbd_in = kbd_in::new().expect("Failed to initialise reader, try 
                                          running as root.");
    loop {
        match kbd_in.poll() {
            Ok(_) => {
                if kbd_in.pressed(KEY_R) {
                    println!("R has been pressed!");
                }
                if kbd_in.released(KEY_R) {
                    println!("R has been released!");
                }
                if kbd_in.repeated(KEY_R) {
                    println!("R has been repeated!");
                }
            },
            Err(_) => { println!("nothing pressed"); }
        }
    }

}
