use libxdo::XDo;
use rand;
use rand_distr::{Distribution, Normal};
use std::{thread, time};

const ONE_SEC_IN_MICROS: f64 = 1_000_000f64;

/* Clicker that clicks at a normal distribution */
pub struct Clicker {
    doer: XDo,
    sleep_dist: Normal<f64>,
    period_min: f64,
}

impl Clicker {
    pub fn new(freq_mean: f64, period_sd_proportion: f64, max_sds: f64) -> Clicker {
        let period_mean = ONE_SEC_IN_MICROS / freq_mean;
        let period_sd = period_mean * period_sd_proportion;
        Clicker {
            doer: XDo::new(None).expect("Init fail"),
            sleep_dist: Normal::new(period_mean, period_sd).unwrap(),
            period_min: period_mean - period_sd * max_sds,
        }
    }
    pub fn sleep_and_click(&self) {
        self.sleep();
        self.doer.click(1).expect("Failed to click");
    }
    pub fn sleep(&self) {
        let mut sleep_time = self.sleep_dist.sample(&mut rand::thread_rng()).round();
        sleep_time = if sleep_time < self.period_min {
            self.period_min
        } else {
            sleep_time
        };
        let sleep_time = time::Duration::from_micros(sleep_time as u64);
        thread::sleep(sleep_time);
    }
}
