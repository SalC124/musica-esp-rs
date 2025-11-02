pub enum NoteName {
    C(u8),
    CSharp(u8),
    D(u8),
    DSharp(u8),
    E(u8),
    F(u8),
    FSharp(u8),
    G(u8),
    GSharp(u8),
    A(u8),
    ASharp(u8),
    B(u8),
    Rest,
}

pub struct Note {
    pub name: NoteName,
    pub beats: f32,
}

impl Note {
    pub fn new(name: NoteName, beats: f32) -> Self {
        Self { name, beats }
    }
    pub fn to_freq(&self) -> Option<u32> {
        let (note_offset, octave) = match self.name {
            NoteName::C(octave) => (0, octave),
            NoteName::CSharp(octave) => (1, octave),
            NoteName::D(octave) => (2, octave),
            NoteName::DSharp(octave) => (3, octave),
            NoteName::E(octave) => (4, octave),
            NoteName::F(octave) => (5, octave),
            NoteName::FSharp(octave) => (6, octave),
            NoteName::G(octave) => (7, octave),
            NoteName::GSharp(octave) => (8, octave),
            NoteName::A(octave) => (9, octave),
            NoteName::ASharp(octave) => (10, octave),
            NoteName::B(octave) => (11, octave),
            NoteName::Rest => return None,
        };
        let semitones_from_c0 = (octave as i32) * 12 + note_offset;
        let semitones_from_a4 = semitones_from_c0 - 49;
        let frequency = 440.0 * 2_f64.powf(semitones_from_a4 as f64 / 12.0);
        Some(frequency.round() as u32)
    }
    pub fn duration_ms(&self, bpm: u32) -> u32 {
        // A quarter note gets one beat at any BPM
        let quarter_note_ms = 60_000.0 / bpm as f32;
        (self.beats * quarter_note_ms) as u32
    }
}
