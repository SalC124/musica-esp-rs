use std::time::Instant;

use crate::{buzzer::Buzzer, notes::Note};

pub struct MusicPlayer<'a> {
    buzzer: Buzzer<'a>,
    bpm: u32,
    notes: Vec<Note>,
    index: usize,
    note_start: Option<Instant>,
    pub is_playing: bool,
}

impl<'a> MusicPlayer<'a> {
    pub fn new(buzzer: Buzzer<'a>, bpm: u32) -> Self {
        Self {
            buzzer,
            bpm,
            notes: Vec::new(),
            index: 0,
            note_start: None,
            is_playing: false,
        }
    }

    pub fn start_playback(&mut self) {
        self.is_playing = true;
        self.index = 0;
        self.note_start = None;
    }

    pub fn load_song(&mut self, mut notes: Vec<Note>) {
        self.notes.append(&mut notes);
    }

    pub fn update(&mut self) {
        if self.index >= self.notes.len() {
            return; // done playing, so stop playback
        }

        let current_note = &self.notes[self.index];
        let note_duration_ms = current_note.duration_ms(self.bpm);

        if self.note_start.is_none() {
            match current_note.name.freq() {
                Some(freq) => self.buzzer.start_tone(freq),
                None => self.buzzer.no_tone(),
            }
            self.note_start = Some(Instant::now());
        }

        if let Some(start_time) = self.note_start {
            let elapsed = start_time.elapsed().as_millis() as u32;

            if elapsed >= note_duration_ms {
                self.buzzer.no_tone();
                self.index += 1;
                self.note_start = None;
            }

            if self.index >= self.notes.len() {
                self.is_playing = false;
                self.buzzer.no_tone();
            }
        }
    }
}
