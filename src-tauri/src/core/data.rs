use crate::core::Note;
use crate::core::Track as CoreTrack;
use midi_file::file::{Division, Event, Header, MetaEvent, QuarterNoteDivision, Track, TrackEvent};
use midi_file::MidiFile;
use serde::Serialize;

//----------
//-- @data
//----------

#[derive(Debug, Default, Serialize)]
pub struct Data {
  num_of_tracks: u32,
  // pulse per quarter note/division
  // `u14` and thus has the range 1 to 16,383.
  // The default value is 1024.
  division: u16,
  track_1: CoreTrack,
  track_2: CoreTrack,
}

impl Data {
  pub fn get_data_from(&mut self, midi_file: MidiFile) {
    assert_ne!(midi_file.tracks_len(), 0);
    self.set_num_tracks(midi_file.tracks_len());
    //
    let mut tracks = midi_file.tracks();

    //header
    self.read_header(midi_file.header());
    //
    // self.log_header();

    //track-1
    let track_1_tempo = tracks.next().unwrap();
    let track_1_notes = tracks.next().unwrap();
    self.track_1.get_data_from(
      midi_file.header(),
      track_1_tempo,
      track_1_notes
    );
    // self.log_track(&self.track_1);

  }

  pub fn read_header(&mut self, header: &Header) {
    let division = header.division();
    if let Division::QuarterNote(quarter_note_division) = division {
      self.set_division(quarter_note_division.get());
    }
  }

  fn set_division(&mut self, value: u16) {
    self.division = value;
  }

  pub fn set_num_tracks(&mut self, value: u32) {
    self.num_of_tracks = value;
  }

  fn log(self) {
    println!("--------------------");
    println!("data: {:?}", self);
    println!("--------------------");
  }

  pub fn log_header(&self) {
    println!("--------------------");
    println!(
      "data-header: 
      num_of_tracks: {}
      division/ppq: {}
    ",
      self.num_of_tracks, self.division
    );
    println!("--------------------");
  }

  pub fn log_track(&self, track: &CoreTrack) {
    println!("--------------------");
    println!("data-track: {:?}", track);
    println!("--------------------");
  }

  pub fn track_1(&self) -> &CoreTrack {
    &self.track_1
  }
}
