export interface Track {
  id: number,
  /// Tempo Meta-Event
  /// (tt tt tt) 3 bytes that specify the amt of ms per quarter note
  /// eg: 0x07A120 = 500_000 ms
  ///quarter note duration in milisecons
  ///amount of time(ie, microseconds) per quarter note
  tempo: number,
  /// num of clock pulses per quarter note
  /// ticks per beat
  division: number,
  seconds_per_tick: number,
  name: string,
  num_of_notes: number,
  notes: Note[],
  timespans_mt3: number[],
  timespans: number[],
  notes_on: Note[],
  notes_off: Note[],
  notes_names: string[],
  notes_velocities: string[],
  raw_str_vec: string[]
}

export interface Note {
  id: number,
  delta_time: number,
  kind: string,
  channel: string,
  note_number: number,
  name: string,
  seconds_per_tick: number,
  velocity: number,
  /// delta-time: The time difference in ticks between 
  /// the previous MIDI track event and the current one
  /// delta-time in seconds
  delta_time_in_seconds: number,
}
