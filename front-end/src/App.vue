<template>
  <n-message-provider>
    <HomePageVue></HomePageVue>
  </n-message-provider>
</template>

<script setup lang="ts">
import HomePageVue from "@/views/HomePage.vue";
</script>

<style>
#app {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>
