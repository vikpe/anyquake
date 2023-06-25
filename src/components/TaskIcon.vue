<script lang="ts" setup>
import classNames from "classnames";
import { Task, TaskResult, TaskStatus } from "./types";

interface Props {
  task: Task
}

const props = defineProps<Props>();

const icons = {
  idle: "/assets/img/icons8-unchecked-checkbox-48.png",
  inProgress: "/assets/img/icons8-dots-loading.gif",
  success: "/assets/img/icons8-checked-checkbox-48.png",
  fail: "/assets/img/icons8-close-48.png",
}

</script>
<template>
  <pre>{{ JSON.stringify(props, null, 2) }}</pre>
  <img :src="classNames({
  [icons.idle]: task.status === TaskStatus.IDLE,
  [icons.inProgress]: task.status === TaskStatus.IN_PROGRESS,
  [icons.success]: task.outcome === TaskResult.SUCCESS,
  [icons.fail]: task.outcome === TaskResult.FAIL
  })" class="inline mr-2" height="32" width="32">
</template>
