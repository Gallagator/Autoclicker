use std::{thread, time};
use rand_distr::{Normal, Distribution};
use rand;

/* Frequency in Hz*/
const FREQ_MEAN : f64 = 19.5f64;
const FREQ_VAR  : f64 = 25f64;
/* period in uS*/
const ONE_SEC_IN_MICRO : f64 = 1_000_000f64;
const PERIOD_MEAN : f64 =  ONE_SEC_IN_MICRO / FREQ_MEAN;
const PERIOD_VAR : f64 = ONE_SEC_IN_MICRO / FREQ_VAR;
const PERIOD_MIN : f64 = PERIOD_MEAN / 4f64;
fn main() {
    let clicker = libxdo::XDo::new(None).expect("Init fail");
    let normal = Normal::new(PERIOD_MEAN, PERIOD_VAR.sqrt()).unwrap();
    let one_sec = time::Duration::from_micros(5000000);

    thread::sleep(one_sec);
    let mut i = 0;
    while i < 1000 {
        let mut sleep_time = normal
                                   .sample(&mut rand::thread_rng())
                                   .round();
        if sleep_time < PERIOD_MIN { 
            sleep_time = PERIOD_MIN;
        }
        let sleep_time = time::Duration::from_micros(sleep_time as u64);
        thread::sleep(sleep_time);
        clicker.click(1).expect("Failed to click");    
        i += 1;
    }
}
