<template>
  <div class="midi-reader flex-col-center w-full">
    <a-button type="primary" @click="on_click" class="button flex-y-center">
      <div class="flex-center gap-8 w-full h-full">
        <Icon icon="solar:upload-track-2-bold" class="mt3-reader__icon" />
        <Icon icon="simple-icons:midi" class="mt3-reader__icon" />
      </div>
    </a-button>

    <h3> </h3>
    <a-textarea class="text-area" v-model:value="midi_info.output_str" placeholder="timespan between notes" auto-size />
    <a-textarea v-model:value="song_info" placeholder="song info" auto-size />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Icon } from "@iconify/vue";
import { Note, Track } from "@/types";

//---------
// @props
//---------
interface Props {
  game_id: string;
}

const props = withDefaults(defineProps<Props>(), {
  game_id: "1",
});

//---------
// @events
//---------
const song_info = ref("");

interface NotesData {
  on_list: Note[]
  off_list:Note[]
  timespans: string[]
  names: string[]
  velocities: string[] 
  raw_str_arr: string[]
}

const notes_data = reactive<NotesData>({
  on_list: new Array(),
  off_list: new Array(),
  timespans: new Array(),
  names: new Array(),
  velocities: new Array(),
  raw_str_arr: new Array()
})

const midi_info = reactive({
  name: "",
  raw_str: "",
  output_str: "",
  num_of_notes: "",
  timespan_min: "",
  timespan_max: "",
});

const on_click = async () => {
  const f = await open();
  console.log("read midi file: " + f);

  invoke("read_midi", {filePathStr: f}).then((res) => {
    if (res) {
      let track = res as Track;
      let notes = track.notes;
      get_notes_data(notes);

      let arr = notes_data.timespans.map((x) => parseFloat(x));
      midi_info.name = typeof f == "string" ? f.replace(/^.*[\\\/]/, "") : "";
      midi_info.num_of_notes = notes_data.timespans.length.toString();
      midi_info.timespan_min = Math.min(...arr).toString();
      midi_info.timespan_max = Math.max(...arr).toString();
      midi_info.raw_str = notes_data.raw_str_arr.join(",");
      song_info.value = `
        name: ${midi_info.name}
        num_of_notes: ${midi_info.num_of_notes}
        timespan(min - max) value: ${midi_info.timespan_min} - ${midi_info.timespan_max}
      `;
    }  
  });
};

const get_notes_data = (notes: Note[]) => {
  let notes_on = notes.filter(x => x.kind == "On");
  let notes_off = notes.filter(x => x.kind == "Off");
  notes_data.on_list = notes_on;
  notes_data.off_list = notes_off;

  console.assert(
    notes_on.length == notes_off.length,
    "notes on and notes off array must have same length"
  );
  // timespans 
  let timespans: string[] = new Array()
   notes_on.forEach((note_on,id) => {
    let timespan = 
    note_on.delta_time_in_seconds + notes_off[id].delta_time_in_seconds;
    timespans.push(timespan.toString());
  });

  timespans = timespans.filter(e => parseFloat(e) > 0.01)
  notes_data.timespans = timespans;

  // note_number
  let notes_name: string[] = new Array()
  notes_on.forEach(note_on => {
https://www.inspiredacoustics.com/en/MIDI_note_numbers_and_center_frequencies
    to_note_name(note_on.note_number);
    notes_name.push(note_on.note_number.toString());
  });
  notes_data.names = notes_name;
  
  //velocity
  let velocities: string[] = new Array()
  notes_on.forEach(note_on => {
    velocities.push(note_on.velocity.toString());
  });
  notes_data.velocities = velocities;

  //
  let raw_str: string[] = new Array();
  timespans.forEach((t, i) => {
    let id = i.toString();
    let number = notes_name[i];
    let velocity = velocities[i]; 
    raw_str.push(`id:${id}-t:${t}-n:${number}-v:${velocity}`);
  })
  notes_data.raw_str_arr = raw_str;
  //

}

const to_note_name = (num: number): string => {
  switch(num) {
    case 96:
      return "C7";
    default:
      return "";
  }
}

</script>

<style lang="scss">
.midi-reader {
  .button {
    &.ant-btn {
      height: 40px;
      padding: 28px 30px;
      margin: 20px auto;
    }
  }

  .mt3-reader__icon {
    font-size: 40px;
  }
}
</style>
