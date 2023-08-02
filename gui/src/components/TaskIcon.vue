<script lang="ts" setup>
import classNames from "classnames";
import { TaskOutcome, TaskStatus } from "./types";

interface Props {
  status: TaskStatus;
  outcome: TaskOutcome;
}

const props = defineProps<Props>();

const icons = {
  idle: "/assets/img/icons8-unchecked-checkbox-48.png",
  inProgress: "/assets/img/icons8-dots-loading.gif",
  success: "/assets/img/icons8-checked-checkbox-48.png",
  fail: "/assets/img/icons8-close-48.png",
};
</script>
<template>
  <!--  <pre>{{ JSON.stringify(props, null, 2) }}</pre>-->
  <img
    :src="
      classNames({
        [icons.idle]: status === TaskStatus.IDLE,
        [icons.inProgress]: status === TaskStatus.IN_PROGRESS,
        [icons.success]: outcome === TaskOutcome.SUCCESS,
        [icons.fail]: outcome === TaskOutcome.FAIL,
      })
    "
    class="inline mr-2"
    height="32"
    width="32"
  />
</template>
