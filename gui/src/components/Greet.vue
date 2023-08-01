<script lang="ts" setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
//import { download } from "tauri-plugin-upload-api";
import { fs, os } from "@tauri-apps/api";
import TaskIcon from "./TaskIcon.vue";
import {
  InstallationTask,
  QuakeInstallation,
  TaskOutcome,
  TaskStatus,
} from "./types";

const downloadMsg = ref("click to download ezQuake");
const progressRef = ref({
  totalProgress: 0,
  totalBytes: 0,
});

const infoTask = ref<InstallationTask>({
  status: TaskStatus.IDLE,
  outcome: TaskOutcome.UNDEFINED,
  data: [],
});

async function onGetInfoClick() {
  infoTask.value.status = TaskStatus.IN_PROGRESS;
  const result: QuakeInstallation[] = await invoke("get_quake_info", {
    needle: "pak0.pak",
  });
  infoTask.value.data = result;
  infoTask.value.status = TaskStatus.COMPLETED;
  infoTask.value.outcome =
    result.length > 0 ? TaskOutcome.SUCCESS : TaskOutcome.FAIL;
}

async function downloadEzQuake() {
  const ezquakeUrl =
    "https://github.com/QW-Group/ezquake-source/releases/download/3.6.2/ezquake.exe";
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
  <div>
    <button class="bg-sky-600 text-white rounded p-3" @click="onGetInfoClick">
      Find Quake Installations
    </button>

    <hr class="my-2" />

    <TaskIcon :task="infoTask" />
    <span class="font-bold">QuakeDirPath</span>:
    <span class="font-mono">{{ JSON.stringify(infoTask.data, null, 2) }}</span>
  </div>

  <hr class="my-6" />

  <button class="bg-blue-600 text-white p-3" @click="downloadEzQuake">
    Download ezQuake
  </button>
  {{ downloadMsg }}

  <hr class="my-6" />
  {{ infoTask }}
  <hr class="my-6" />
  {{ bytesInMb(progressRef.totalProgress) }} of
  {{ bytesInMb(progressRef.totalBytes) }}
</template>
