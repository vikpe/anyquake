<script lang="ts" setup>
import { ref } from "vue";
import TaskIcon from "./TaskIcon.vue";
import { InvokeTask } from "./task";

const counter = ref(0);
const onTaskChange = () => {
  counter.value += 1;
};

const infoTask = ref<InvokeTask>(new InvokeTask("get_quake_info", onTaskChange));
const uninstallEzquakeTask = ref<InvokeTask>(
  new InvokeTask("uninstall_ezquake", onTaskChange),
);
</script>

<template>
  {{ counter }}

  <div>
    <button class="bg-sky-600 text-white rounded p-3" @click="infoTask.run">
      Find Quake Installations
    </button>

    <hr class="my-2" />

    <TaskIcon :task="infoTask" />
    <span class="font-bold">QuakeDirPath</span>:
    <span class="font-mono">{{ JSON.stringify(infoTask.data, null, 2) }}</span>
  </div>

  <hr class="my-6" />

  <div>
    <button
      class="bg-sky-600 text-white rounded p-3"
      @click="uninstallEzquakeTask.run"
    >
      Uninstall ezQuake
    </button>

    <hr class="my-2" />

    <TaskIcon :task="uninstallEzquakeTask" />
    <span class="font-mono">{{
      JSON.stringify(uninstallEzquakeTask.data, null, 2)
    }}</span>
  </div>
</template>
