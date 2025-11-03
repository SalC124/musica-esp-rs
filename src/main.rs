use esp_idf_svc::hal::{
    delay::FreeRtos,
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
    peripherals::Peripherals,
};
use musica_esp_rs::{buzzer::Buzzer, notes::{Note, NoteName}};
use musica_esp_rs::notes::NoteName as N;
use musica_esp_rs::note;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let pin = peripherals.pins.gpio2;
    let ledc = peripherals.ledc;

    let mut buzzer = Buzzer::new(pin.into(), ledc);

    let _note = Note::new(NoteName::C(4), 1.0);
    let _knote = NoteName::E(4).beats(1.0);
    let _freakwhensea = NoteName::D(4).beats(2.0).beats;
    let _marco = note!{ C 4 | 1};

    // let bpm = 60;
    // let note = Note::new(NoteName::C(4), 1.0);
    //
    // buzzer.start_tone(note.to_freq().unwrap());
    // FreeRtos::delay_ms(note.duration_ms(bpm));
    // buzzer.no_tone();
    // buzzer.start_tone(Note::new(NoteName::G(4), 0.5).to_freq().unwrap());
    // FreeRtos::delay_ms(500);
    // buzzer.no_tone();

//     let freq = 466.2;
//     let dur = 1000;
//
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
