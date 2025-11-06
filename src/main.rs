use esp_idf_svc::hal::{delay::FreeRtos, peripherals::Peripherals};
use musica_esp_rs::{
    buzzer::Buzzer,
    music_player::MusicPlayer,
    note,
    notes::{Note, NoteName},
};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let pin = peripherals.pins.gpio2;
    let ledc = peripherals.ledc;

    let buzzer = Buzzer::new(pin.into(), ledc);
    let bpm = 160;

    let mut music_player = MusicPlayer::new(buzzer, bpm);

    let melody = vec![
        note! {D 4 | 1},
        note! {F 4 | 1},
        note! {G 4 | 1},
        note! {GSharp 4 | 1},
        note! {G 4 | 1},
        note! {F 4 | 1},
        note! {D 4 | 1},
        note! {R 4 | 2},
        note! {C 4 | 0.5},
        note! {E 4 | 0.5},
        note! {D 4 | 1},
    ];

    // let scale = vec![
    //     note! {C 4 | 1},
    //     note! {D 4 | 1},
    //     note! {E 4 | 1},
    //     note! {F 4 | 1},
    //     note! {G 4 | 1},
    //     note! {A 4 | 1},
    //     note! {B 4 | 1},
    //     note! {C 5 | 1},
    // ];

    music_player.load_song(melody);
    music_player.start_playback();

    loop {
        music_player.update();
        FreeRtos::delay_ms(10);

        if !music_player.is_playing {
            break;
        }
    }

    // let bpm = 60;
    // let note = Note::new(NoteName::C(4), 1.0);
    //
    // buzzer.start_tone(note.name.freq().unwrap());
    // FreeRtos::delay_ms(note.duration_ms(bpm));
    // buzzer.no_tone();
    // buzzer.start_tone(Note::new(NoteName::G(4), 0.5).name.freq().unwrap());
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
