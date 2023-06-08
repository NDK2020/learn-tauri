// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use midi_file::MidiFile;
use std::{error::Error, fs, fs::File, path::Path};

mod core;
use crate::core::{Data, Note};

mod libs;

// lsof -i @localhost
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![process_file, read_midi_mt3])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn process_file(file_path: &str) -> String {
  println!("process_file {}", file_path);
  format!("file {}! !", file_path)
}
#[derive(serde::Serialize)]
struct Mt3Response {
  message: String,
  data: Vec<f32>,
}

#[tauri::command]
fn read_midi_mt3(file_path_str: &str) -> String {
  let midi_file = read_midi_file(file_path_str.to_string());
  let mut data = Data::default();
  data.get_data_from(midi_file);
  let tmp: Vec<String> = data.track_1().timespans_mt3().clone()
    .iter()
    .map(|x| x.to_string()).collect();

  tmp.join(",")
  // data.track_1().timespans_mt3().clone()
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

