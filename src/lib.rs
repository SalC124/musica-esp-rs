use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyOutputPin, OutputPin},
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, LEDC},
    peripheral::Peripheral,
    peripherals::Peripherals,
    units::Hertz,
};

pub mod buzzer;
pub mod notes;
