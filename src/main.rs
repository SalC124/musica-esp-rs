use esp_idf_svc::hal::{
    delay::{self, FreeRtos},
    gpio::PinDriver,
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
    peripherals::Peripherals,
    units::Hertz,
};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let freq = 466.2;
    let dur = 1000;

    let buzzer = peripherals.pins.gpio2;
    let ledc = peripherals.ledc;

    let timer_config = TimerConfig::default().frequency((freq as u32).into());
    let timer = LedcTimerDriver::new(ledc.timer0, &timer_config).unwrap();
    let mut driver = LedcDriver::new(ledc.channel0, timer, buzzer).unwrap();
    let max_duty = driver.get_max_duty();

    driver.set_duty(max_duty / 2).unwrap();
    println!("Playing B♭ with PWM for {} ms", dur);
    FreeRtos::delay_ms(dur);
    driver.set_duty(0).unwrap();
    println!("PWM tone complete");

    // watchdog errors
    // {
    //     let mut buzzer = PinDriver::output(peripherals.pins.gpio2).unwrap();
    //     buzzer.set_low().unwrap();
    //
    //     loop {
    //         thread::sleep(Duration::from_nanos((1_000_000_000_f64 / freq) as u64));
    //         buzzer.set_high().unwrap();
    //         thread::sleep(Duration::from_nanos(2145002));
    //         buzzer.set_low().unwrap();
    //     }
    // }
}
