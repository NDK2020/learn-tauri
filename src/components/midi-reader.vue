<template>
  <div class="midi-reader flex-col-center w-full">
    <a-button type="primary" @click="on_click" class="button flex-y-center">
      <div class="flex-center gap-8 w-full h-full">
        <Icon icon="solar:upload-track-2-bold" class="mt3-reader__icon" />
        <Icon icon="simple-icons:midi" class="mt3-reader__icon" />
      </div>
    </a-button>

    <a-textarea class="text-area" v-model:value="midi_info.str" placeholder="timespan between notes" auto-size />
    <a-textarea v-model:value="song_info" placeholder="song info" auto-size />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Icon } from "@iconify/vue";

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
const song_str = ref();
const song_info = ref("");
const midi_info = reactive({
  name: "",
  str: "",
  num_of_notes: "",
  timespan_min: "",
  timespan_max: "",
});

const on_click = async () => {
  const f = await open();
  console.log(f);
  invoke("read_midi_mt3", { filePathStr: f }).then((res) => {
    if (!res) return;
    midi_info.name = typeof f == "string" ? f.replace(/^.*[\\\/]/, "") : "";
    midi_info.str = res.toString();
    let arr = midi_info.str.split(",").map((x) => parseFloat(x));
    midi_info.num_of_notes = arr.length.toString();
    midi_info.timespan_min = Math.min(...arr).toString();
    midi_info.timespan_max = Math.max(...arr).toString();
    song_info.value = `
      name: ${midi_info.name}
      num_of_notes: ${midi_info.num_of_notes}
      timespan(min - max): ${midi_info.timespan_min} - ${midi_info.timespan_max}
    `;
  });

  invoke("read_midi", {filePathStr: f}).then((res) => {
    console.log(res);
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
