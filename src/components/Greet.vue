<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
//import { download } from "tauri-plugin-upload-api";
import { fs, os } from "@tauri-apps/api";

const greetMsg = ref("");
const downloadMsg = ref("click to download ezQuake");
const name = ref("");
const progressRef = ref({
  totalProgress: 0,
  totalBytes: 0,
})

const pakDirPath = ref({  });

onMounted(async () => {
  pakDirPath.value = await invoke("get_quake_info", { needle: "pak0.pak" });
})

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function downloadEzQuake() {
  const ezquakeUrl = "https://github.com/QW-Group/ezquake-source/releases/download/3.6.2/ezquake.exe";
  downloadMsg.value = await invoke("download", { url: ezquakeUrl });

  const tmpDir = await os.tempdir();
  const targetDirPath = `${tmpDir}/anyquake`;
  const targetFilePath = `${targetDirPath}/ezquake.exe`;
  await fs.createDir(targetDirPath, { recursive: true });
  console.log("targetFilePath", targetFilePath);

  // await download(
  //     ezquakeUrl,
  //     targetFilePath,
  //     (progress: number, total: number) => {
  //       console.log(`Downloaded ${progress} of ${total} bytes`);
  //       progressRef.value = {
  //         totalProgress: progressRef.value.totalProgress + progress,
  //         totalBytes: total
  //       }
  //     }
  // );
}

function bytesInMb(bytes: number): string {
  return `${Math.round(bytes / 1024 / 1024)} mb`;
}

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" class="border-2 p-2 text-black" placeholder="Enter a name..." />
    <button class="bg-gray-400 p-2" type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <hr class="my-6">

  <pre>
  QuakeDirPath: {{ pakDirPath }}
  </pre>

  <hr class="my-6">

  <button class="bg-blue-600 text-white p-3" @click="downloadEzQuake">Download ezQuake</button>
  {{ downloadMsg }}

  <hr class="my-6">
  {{ pakDirPath }}
  <hr class="my-6">
  {{ bytesInMb(progressRef.totalProgress) }} of {{ bytesInMb(progressRef.totalBytes) }}
</template>
