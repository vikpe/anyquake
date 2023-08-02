<script lang="ts" setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import TaskIcon from "./TaskIcon.vue";
import {Task, TaskOutcome, TaskStatus,} from "./types";


const infoTask = ref<Task>({
  status: TaskStatus.IDLE,
  outcome: TaskOutcome.UNDEFINED,
  data: [],
});

const installEzquakeTask = ref<Task>({
  status: TaskStatus.IDLE,
  outcome: TaskOutcome.UNDEFINED,
  data: [],
});

async function onGetInfo() {
  infoTask.value.status = TaskStatus.IN_PROGRESS;
  const result: string[] = await invoke("get_quake_info");
  infoTask.value.data = result;
  infoTask.value.status = TaskStatus.COMPLETED;
  infoTask.value.outcome =
      result.length > 0 ? TaskOutcome.SUCCESS : TaskOutcome.FAIL;
}

async function onUnintallEzquake() {
  installEzquakeTask.value.status = TaskStatus.IN_PROGRESS;
  const result: string[] = await invoke("uninstall_ezquake");
  installEzquakeTask.value.data = result;
  installEzquakeTask.value.status = TaskStatus.COMPLETED;
  installEzquakeTask.value.outcome =
      result.length > 0 ? TaskOutcome.SUCCESS : TaskOutcome.FAIL;
}

</script>

<template>
  <div>
    <button class="bg-sky-600 text-white rounded p-3" @click="onGetInfo">
      Find Quake Installations
    </button>

    <hr class="my-2"/>

    <TaskIcon :task="infoTask"/>
    <span class="font-bold">QuakeDirPath</span>:
    <span class="font-mono">{{ JSON.stringify(infoTask.data, null, 2) }}</span>
  </div>

  <hr class="my-6"/>

  <div>
    <button class="bg-sky-600 text-white rounded p-3" @click="onUnintallEzquake">
      Uninstall ezQuake
    </button>

    <hr class="my-2"/>

    <TaskIcon :task="installEzquakeTask"/>
    <span class="font-mono">{{ JSON.stringify(installEzquakeTask.data, null, 2) }}</span>
  </div>
</template>
