mod clicker;
use clicker::clicker;

/* Frequency in Hz*/
const FREQ_MEAN : f64 = 19.5f64;
const FREQ_SD   : f64 = 5f64;
const FREQ_MIN  : f64 = 10f64;

fn main() {
    let clicker = clicker::new(FREQ_MEAN, FREQ_SD, FREQ_MIN);  
}
