<template>
  <div class="home content-width-boxed">
    <!-- <h1>Welcome to ama-creative-tools !</h1> -->

    <div class="flex-center">
      <a href="https://www.amanotes.com/" target="_blank">
        <img src="/ama-logo--white-400x100.svg" class="ama-logo" alt="amanotes logo" />
      </a>
    </div>

    <div class="flex-center">
      <midi-reader-mt3 />
    </div>
  </div>
</template>

<script setup lang="ts">
// sudo sysctl fs.inotify.max_user_watches=131070
// sudo sysctl -p
// sudo sysctl -p
</script>

<style lang="scss">
@import "@/styles/index.scss";

</style>
