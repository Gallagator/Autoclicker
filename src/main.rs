mod clicker;
use clicker::clicker::Clicker;

/* Frequency in Hz*/
const FREQ_MEAN: f64 = 1000f64;
const PERIOD_SD_PROPORTION: f64 = 0.1f64;
const MAX_SDS: f64 = 3f64;

fn main() {
    let clicker = Clicker::new(FREQ_MEAN, PERIOD_SD_PROPORTION, MAX_SDS);
    let mut i = 0;
    while i < 10000 {
        clicker.sleep_and_click();
        i += 1;
    }
}
