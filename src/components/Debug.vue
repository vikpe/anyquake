<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { os } from "@tauri-apps/api";

const env = ref({
  type: "",
  platform: "",
  tempdir: "",
  arch: "",
  version: "",
});

onMounted(async () => {
  env.value = {
    type: await os.type(),
    platform: await os.platform(),
    arch: await os.arch(),
    version: await os.version(),
    tempdir: await os.tempdir(),
  };
});
</script>

<template>
  <div class="p-3 text-sm bg-black font-mono mb-8">
    <pre>{{ JSON.stringify(env, null, 2) }}</pre>
  </div>
</template>
