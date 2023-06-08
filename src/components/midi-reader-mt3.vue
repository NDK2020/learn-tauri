<template>
  <div class="flex-col-center">
    <a-button type="primary" @click="on_click" class="button flex-col-center">
      <Icon icon="fa6-solid:file-import" class="mt3-reader__icon" />
    </a-button>
    <a-textarea v-model:value="song_str" placeholder="timespan between notes" auto-size />
  </div>
</template>

<script setup lang="ts">
import { APP_NAME } from "@/constants";
import * as tauri_path from "@tauri-apps/api/path";
import * as os from "@tauri-apps/api/os";
import * as fs from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
// import { message } from "ant-design-vue";
import { open } from "@tauri-apps/api/dialog";
import { Icon } from "@iconify/vue";

//--------
// @file
//--------
const download_dir = ref();
const os_type = ref();
const file_seperator = ref();
const app_dir = ref();

onMounted(async () => {
  download_dir.value = await tauri_path.downloadDir();
  os_type.value = await os.type();
  file_seperator.value = os_type.value === "Windows_NT" ? "\\" : "/";
  await fs.createDir(APP_NAME, {
    recursive: true,
    dir: fs.BaseDirectory.Download,
  });
  //
  app_dir.value = `${download_dir.value}${APP_NAME}`;
  console.log(download_dir.value);
  console.log(os_type.value);
  console.log(app_dir.value);
});

//----------
// @events
//----------
const song_str = ref();
const on_click = async () => {
  const f = await open();
  console.log(f);
  invoke("read_midi_mt3", { filePathStr: f }).then((res) => {
    if (res) {
      song_str.value = res;
      console.log(song_str.value);
    }
  });
  // invoke("process_file", {name: "hello anh em"});
};
</script>

<styles lang="scss">
.button {
  &.ant-btn {
    height: 40px;
    padding: 20px 40px;
    display: block;
    margin: 20px auto;
  }
}

.mt3-reader__icon {
  font-size: 20px;
  display: block;
  margin-top: -10px;
}
</styles>
