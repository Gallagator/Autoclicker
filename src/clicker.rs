use std::{thread, time};
use rand_distr::{Normal, Distribution};
use rand;
use libxdo::XDo;


mod clicker {
    const ONE_SEC_IN_MICROS : f64 = 1_000_000f64;
    /* Clicker that clicks at a normal distribution */
    pub struct clicker {
        doer       : Xdo,
        sleep_dist : Normal,
        period_min : f64,
    }

    impl clicker {
        pub fn new(freq_mean : f64, freq_sd : f64, freq_min : f64) -> clicker {
            let period_mean = ONE_SEC_IN_MICROS / freq_mean;
            let period_sd   = ONE_SEC_IN_MICROS / freq_sd;
            struct clicker {
                Xdo        : XDo::new(None).expect("Init fail"),
                normal     : Normal::new(period_mean, period_sd),
                period_min : ONE_SEC_IN_MICROS / freq_min,
            }
        }
        pub fn sleep_and_click(&self) {
            let mut sleep_time = self.sleep_dist
                                 .sample(&mut rand::thread_rng())
                                 .round();
            sleep_time = if sleep_time < self.period_min { self.period_min } 
                         else { sleep_time };
            let sleep_time = time::Duration::from_micros(sleep_time as u64);
            thread::sleep(sleep_time);
            self.doer.click(1).expect("Failed to click");    
        }
    }
    
}
