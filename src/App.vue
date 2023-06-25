<script lang="ts" setup>
import Greet from "./components/Greet.vue";
import Debug from "./components/Debug.vue";
</script>

<template>
  <div class="container border-4 border-red-400">

    <div class="flex items-center p-4 space-x-4">
      <img alt="anyquake" class="h-24" src="/assets/img/quake_ranger.jpg" />
      <div>
        <h1 class="font-bold text-4xl">Welcome to anyquake!</h1>
        <p class="text-lg">Select components to install.</p>
      </div>
    </div>

    <hr class="border-t my-6">

    ezQuake

    <hr class="border-t my-6">

    <Greet />

    <hr class="border-t my-6">

    <Debug />
  </div>
</template>
