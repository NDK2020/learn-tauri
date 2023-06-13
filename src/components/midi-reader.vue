<template>
  <div class="midi-reader flex-col-center w-full">
    <a-button type="primary" @click="on_click" class="button flex-y-center">
      <div class="flex-center gap-8 w-full h-full">
        <Icon icon="solar:upload-track-2-bold" class="mt3-reader__icon" />
        <Icon icon="simple-icons:midi" class="mt3-reader__icon" />
      </div>
    </a-button>

    <div class="title-wrapper w-full mt-[16px] mb-[4px]"><h4> Copy This Content </h4></div>
    <a-textarea class="text-area" v-model:value="midi_info.output_str"
      placeholder="output" auto-size />

    
    <div class="title-wrapper w-full mt-[16px] mb-[4px]"><h4> Song Info</h4></div>
    <a-textarea class="text-area" v-model:value="song_info" placeholder="song info" auto-size />

    
    <div class="title-wrapper w-full mt-[16px] mb-[4px]"><h4> Raw Content </h4></div>
    <a-textarea class="text-area" v-model:value="midi_info.raw_str" placeholder="raw string" auto-size />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Icon } from "@iconify/vue";
import { Track } from "@/types";

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
      // let notes = track.notes;
      // get_notes_data(notes);

      midi_info.name = typeof f == "string" ? f.replace(/^.*[\\\/]/, "") : "";
      midi_info.num_of_notes = track.timespans.length.toString();
      midi_info.timespan_min = Math.min(...track.timespans).toString();
      midi_info.timespan_max = Math.max(...track.timespans).toString();
      midi_info.raw_str = track.raw_str_vec.join(",");
      song_info.value = `
        name: ${midi_info.name}
        num_of_notes: ${midi_info.num_of_notes}
        timespan(min - max) value: 
            ${midi_info.timespan_min} - ${midi_info.timespan_max}
      `;

      switch(props.game_id) {
        case "1":
          midi_info.output_str = track.timespans.map(t => t.toString()).join(",")
          break;
        case "2":
          midi_info.output_str = midi_info.raw_str
          break;
        default:
          midi_info.output_str = ""
          break;
      }
    }  
  });
};


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
