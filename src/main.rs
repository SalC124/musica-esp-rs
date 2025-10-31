use esp_idf_svc::hal::{
    delay::FreeRtos,
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
    peripherals::Peripherals,
};
use musica_esp_rs::buzzer::Buzzer;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let freq = 466.2;
    let dur = 1000;

    let pin = peripherals.pins.gpio2;
    let ledc = peripherals.ledc;

    let mut buzzer = Buzzer::new(pin.into(), ledc);

    buzzer.start_tone(freq as u32);
    FreeRtos::delay_ms(dur);
    buzzer.no_tone();

//     let timer_config = TimerConfig::default().frequency((freq as u32).into());
//     let timer = LedcTimerDriver::new(ledc.timer0, &timer_config).unwrap();
//     let mut driver = LedcDriver::new(ledc.channel0, timer, buzzer).unwrap();
//     let max_duty = driver.get_max_duty();
//
//     driver.set_duty(max_duty / 2).unwrap();
//     println!("Playing B♭ with PWM for {} ms", dur);
//     FreeRtos::delay_ms(dur);
//     driver.set_duty(0).unwrap();
//     println!("PWM tone complete");
}
