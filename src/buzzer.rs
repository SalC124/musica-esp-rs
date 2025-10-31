use esp_idf_svc::hal::{
    gpio::{AnyOutputPin, Pin},
    ledc::{config::TimerConfig, LedcDriver, LedcTimer, LedcTimerDriver, LEDC, TIMER0},
    // prelude::*,
    units::{FromValueType, Hertz},
};

pub struct Buzzer<'a> {
    pin_num: i32,
    driver: Option<LedcDriver<'a>>,
    timer: Option<LedcTimerDriver<'static, TIMER0>>,
}

impl<'a> Buzzer<'a> {
    pub fn new(pin: AnyOutputPin, ledc: LEDC) -> Self {
        let pin_num = pin.pin();

        Self {
            driver: None,
            timer: None,
            pin_num,
        }
    }

    pub fn start_tone(&mut self, freq_hz: u32) {
        self.driver = None;
        self.timer = None;

        let pin = unsafe { AnyOutputPin::new(self.pin_num) };
        let ledc = unsafe { LEDC::new() };

        let timer_config = TimerConfig::default().frequency(freq_hz.Hz());
        let timer = LedcTimerDriver::new(ledc.timer0, &timer_config).unwrap();

        let mut driver = LedcDriver::new(ledc.channel0, &timer, pin).unwrap();

        let max_duty = driver.get_max_duty();
        driver.set_duty(max_duty / 2).unwrap();

        self.timer = Some(timer);
        self.driver = Some(driver);
    }

    pub fn no_tone(&mut self) {
        if let Some(mut driver) = self.driver.take() {
            driver.set_duty(0).unwrap();
        }
        self.timer = None;
        self.driver = None; // should already be `None`, but just in case
    }
}
