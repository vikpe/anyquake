<script lang="ts" setup>
import { ref } from "vue";
import TaskIcon from "./TaskIcon.vue";
import { CommandTask, InvokeTask } from "./task";

const counter = ref(0);
const onTaskChange = () => {
  counter.value += 1;
};

const infoTask = ref<InvokeTask>(
  new InvokeTask("get_quake_info", onTaskChange),
);
const modules = ["afterquake", "ezquake"];

const tasks = ref<CommandTask[]>([]);

function runCommand(name, module_id) {
  let task = new CommandTask({ name, module_id }, onTaskChange);
  tasks.value.push(task);
  task.run();
}
</script>

<template>
  {{ counter }}

  <hr />

  <table class="w-[600px] text-left table-auto">
    <thead>
      <tr>
        <th>Name</th>
        <th>Install</th>
        <th>Uninstall</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="module_id in modules" :key="module_id">
        <td>{{ module_id }}</td>
        <td>
          <button
            class="bg-sky-600 text-white rounded p-2"
            @click="() => runCommand('Install', module_id)"
          >
            Install
          </button>
        </td>
        <td>
          <button
            class="bg-sky-600 text-white rounded p-2"
            @click="() => runCommand('Uninstall', module_id)"
          >
            Uninstall
          </button>
        </td>
      </tr>
    </tbody>
  </table>

  <hr />

  <table class="min-w-[500px] text-left border">
    <thead>
      <tr>
        <th class="p-2">Command</th>
        <th class="p-2">Module</th>
        <th class="p-2">Status</th>
        <th class="p-2">Outcome</th>
        <th class="p-2">Data</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="task in tasks.slice().reverse()">
        <td class="p-2">{{ task.command.name }}</td>
        <td class="p-2">{{ task.command.module_id }}</td>
        <td class="p-2">{{ task.status }}</td>
        <td class="p-2">{{ task.outcome }}</td>
        <td class="p-2">{{ JSON.stringify(task.data, null, 2) }}</td>
      </tr>
    </tbody>
  </table>

  <hr />

  <div>
    <button class="bg-sky-600 text-white rounded p-3" @click="infoTask.run">
      Find Quake Installations
    </button>

    <hr />

    <TaskIcon :task="infoTask" />
    <span class="font-bold">QuakeDirPath</span>:
    <span class="font-mono">{{ JSON.stringify(infoTask.data, null, 2) }}</span>
  </div>
</template>
