// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use serde::{ser::Serializer, Deserialize, Serialize};

use midi_file::MidiFile;
use std::{error::Error, fs, fs::File, path::Path};

mod core;
use crate::core::{Data, Note, Track};

mod libs;
use crate::libs::custom_macros as cm;

// lsof -i @localhost
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      read_midi_gduc,
      read_midi_mt3,
      read_midi
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn read_midi_gduc(file_path_str: &str) -> String {
  let midi_file = read_midi_file(file_path_str.to_string());
  let mut data = Data::default();
  data.get_data_from(midi_file);
  let tmp: Vec<String> = data
    .track_1()
    .timespans_mt3()
    .clone()
    .iter()
    .filter(|&&x| !cm::is_approx!(x as f64, 0.0, 1e-2))
    .map(|x| x.to_string())
    .collect();
  matches!(0, 0);
  tmp.join(",")
}

#[tauri::command]
fn read_midi_mt3(file_path_str: &str) -> String {
  let midi_file = read_midi_file(file_path_str.to_string());
  let mut data = Data::default();
  data.get_data_from(midi_file);
  let tmp: Vec<String> = data
    .track_1()
    .timespans_mt3()
    .clone()
    .iter()
    .filter(|&&x| !cm::is_approx!(x as f64, 0.0, 1e-2))
    .map(|x| x.to_string())
    .collect();

  tmp.join(",")
}

// create the error type that represents all errors possible in our program
// #[derive(Debug, thiserror::Error)]
// pub enum CommandError {
//   #[error(transparent)]
//   Io(#[from] std::io::Error),
// }
//
// // we must manually implement serde::Serialize
// impl serde::Serialize for Error {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::ser::Serializer,
//   {
//     serializer.serialize_str(self.to_string().as_ref())
//   }
// }


// #[derive(Debug, Deserialize, Serialize)]
// struct MidiResponse {
//   message: String,
//   track: Track,
// }

#[tauri::command]
fn read_midi(file_path_str: &str) -> Track {
  let midi_file = read_midi_file(file_path_str.to_string());
  let mut data = Data::default();
  data.get_data_from(midi_file);
  data.track_1().clone()
  // Ok(data.track_1().clone())
}

fn run() {
  let file_path_str = get_file_path();
  println!("file_path: {} ", file_path_str);
  //
  let midi_file = read_midi_file(file_path_str);
  let mut data = Data::default();
  data.get_data_from(midi_file);
}

fn get_file_path() -> String {
  let base_path = "./src/assets/";
  let file_name = "Herewithme_playableads_tut";
  let file_extension = ".mid";

  format!("{}{}{}", base_path, file_name, file_extension)
}

fn read_midi_file(file_path_str: String) -> MidiFile {
  let file_path = &Path::new(&file_path_str);
  let mut f = File::open(file_path).unwrap();
  MidiFile::read(f).unwrap()
}
