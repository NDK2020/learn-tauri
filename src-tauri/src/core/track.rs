use crate::core::{Note};
use midi_file::file::{
  Division, Event, Header, MetaEvent, QuarterNoteDivision, Track as MfTrack, TrackEvent,
};
use midi_file::core::Message;

use crate::libs::custom_macros as cm;
use serde::Serialize;


//-----------
//-- @track
//-----------
#[derive(Clone, Debug, Default, Serialize)]
pub struct Track {
  id: i32,
  /// Tempo Meta-Event
  /// (tt tt tt) 3 bytes that specify the amt of ms per quarter note
  /// eg: 0x07A120 = 500_000 ms
  ///quarter note duration in milisecons
  ///amount of time(ie, microseconds) per quarter note
  tempo: u32,
  /// num of clock pulses per quarter note
  /// ticks per beat
  division: u16,
  seconds_per_tick: f32,
  name: String,
  num_of_notes: usize,
  notes: Vec<Note>,
  timespans_mt3: Vec<f32>
}

impl Track {
  pub fn get_data_from(
    &mut self,
    header: &Header,
    track_has_tempo: &MfTrack,
    track_has_notes: &MfTrack
  ) {
    self.get_data_from_header(header);
    self.get_data_from_tracks(track_has_tempo, track_has_notes);
    self.calc_notes_delta_time_in_seconds();
    self.log();
    self.calc_timespans_mt3();
  }

  pub fn get_data_from_header(&mut self, header: &Header) {
    let division = header.division();
    if let Division::QuarterNote(quarter_note_division) = division {
      self.set_division(quarter_note_division.get());
    }
  }

  fn set_division(&mut self, value: u16) {
    self.division = value;
  }

  pub fn get_data_from_tracks(&mut self, track_has_tempo: &MfTrack, track_has_notes: &MfTrack) {
    self.get_tempo(track_has_tempo);
    self.calc_sec_per_tick();
    //
    self.get_name(track_has_notes);
    self.get_notes(track_has_notes);
  }

  pub fn get_tempo(&mut self, track: &MfTrack) {
    let track_events_iter = track.events();

    let mut has_tempo = false;
    track.events().for_each(|te| {
      if let Event::Meta(MetaEvent::SetTempo(ms_per_quarter)) = te.event() {
        self.set_tempo(ms_per_quarter.get());
        has_tempo = true;
      }
    });

    if (!has_tempo) {
      println!("this track has no TEMPO meta event");
    }
  }
  //

  fn set_tempo(&mut self, value: u32) {
    self.tempo = value;
  }

  pub fn get_name(&mut self, track: &MfTrack) {
    let ev_name = track
      .events()
      .find(|e| matches!(e.event(), Event::Meta(MetaEvent::TrackName(_))));

    if let Event::Meta(MetaEvent::TrackName(name)) = ev_name.unwrap().event() {
      self.set_name(name.to_string());
    }
  }

  fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn get_notes(&mut self, track: &MfTrack) {
    let mut notes: Vec<Note> = Vec::new();
    let mut note_id: i32 = 0;
    for track_event in track.events() {
      if !matches!( track_event.event(),
        Event::Midi(Message::NoteOn(_))
      ) {
        continue;
      }

      let mut note = Note::default();


      // println!("{:?}", track_event);
      note.get_data_from(note_id, track_event, self.seconds_per_tick);
      notes.push(note);
      note_id += 1;
    }
    self.set_notes(&notes);
    self.set_num_of_notes(&notes);
  }

  pub fn calc_notes_delta_time_in_seconds(&mut self) {
    if (!self.notes.is_empty()) {
      self.notes
        .iter_mut()
        .for_each(|n| n.calc_delta_time_in_seconds());
    }
  }

  fn set_notes(&mut self, notes: &Vec<Note>) {
    self.notes = notes.to_owned();
  }

  fn set_num_of_notes(&mut self, notes: &Vec<Note>) {
    self.num_of_notes = notes.len();
  }

  pub fn to_bpm(&self, val: u32) {
    // 60.000.000 / (tt tt tt);
    println!("{} to bpm: {}", val, 60_000_000 / val);
  }

  /// ticks_per_quarter = <PPQ(division) from the header>
  /// µs_per_quarter = <Tempo in latest Set Tempo event>
  /// µs_per_tick = µs_per_quarter / ticks_per_quarter
  /// seconds_per_tick = µs_per_tick / 1.000.000
  /// seconds = ticks * seconds_per_tick
  fn calc_sec_per_tick(&mut self) {
    let microsecond_per_tick = self.tempo as f32 / self.division as f32;
    self.seconds_per_tick = microsecond_per_tick / 1_000_000.0;
  }

/// In MIDI, each message that starts a note (a Note On message for a particular channel and key with non-zero velocity) must eventually be followed by a message that ends the same note (a Note On message for the same channel and key with zero velocity, or a Note Off message for the same channel and key with any velocity).
///
/// It is possible to start another instance of a note with the same channel and key as a note that is already sounding, but each instance must eventually have a message that ends the note.
/// However, when there are overlapping notes with the same channel and key, it is ambiguous which end goes with which start. For example, if you encounter the following sequence of messages on the same channel at beat intervals:
///
/// start C5, start C5, end C5, end C5.
///
/// The sequence could be interpreted two ways:
///
/// start a, start b, end b, end a.
/// or
/// start a, start b, end a, end b.
/// Because of this ambiguity, I believe overlapping notes of the same channel and key are usually avoided in MIDI.
  fn calc_timespans_mt3(&mut self) {
    let mut id = 0;

    let notes_on = self.notes
      .iter()
      .filter(|e| e.is_on());

    let notes_off = self.notes
      .iter()
      .filter(|e| !e.is_on());
    assert_eq!(

      notes_on.clone().count(), notes_off.clone().count(),
      "len of notes on/off list must be eq"
    );


    for (note_on, note_off) in notes_on.zip(notes_off) {
      self.timespans_mt3.push(
        note_on.delta_time_in_seconds() + note_off.delta_time_in_seconds()
      )
    }

    self.timespans_mt3.iter().for_each(|e| println!("{}", e));
    println!("len of timespan: {}", self.timespans_mt3.len());

    println!("epsilon: {}", f64::EPSILON);
    // cm::add!(1, 2);
  }

  pub fn timespans_mt3(&self) -> &Vec<f32> {
    &self.timespans_mt3
  }


  fn log(&self) {
    println!("--------------------");
    println!(
      "track: 
      id: {}
      tempo: {}
      name: {}
      num_of_notes: {}
    ",
      self.id, self.tempo, self.name, self.num_of_notes
    );
    self.log_notes();
    println!("--------------------");
  }

  fn log_notes(&self) {
    println!("notes: ");
    self.notes.iter().for_each(|e| println!("{:?}", e));
  }
}
