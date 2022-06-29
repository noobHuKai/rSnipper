<template>
  <div style="height: 100%; position: relative">
    <n-layout position="absolute">
      <n-layout-header style="height: 64px; padding: 0 24px 0 24px" class="flex" bordered>
        <AppBarVue class="self-center"></AppBarVue>
      </n-layout-header>
      <n-layout has-sider position="absolute" style="top: 64px; bottom: 64px">
        <n-layout-sider bordered content-style="padding: 24px;">
          <CatalogSiderVue></CatalogSiderVue>
        </n-layout-sider>
        <n-layout content-style="padding: 24px;">
          <MainContentVue></MainContentVue>
        </n-layout>
      </n-layout>
      <n-layout-footer bordered position="absolute" style="height: 64px; padding: 24px">
        <BottomBarVue></BottomBarVue>
      </n-layout-footer>
    </n-layout>
  </div>
</template>

<script setup lang="ts">
import AppBarVue from '@/components/AppBar.vue';
import CatalogSiderVue from '@/components/CatalogSider.vue';
import MainContentVue from "@/components/MainContent.vue";
import BottomBarVue from "@/components/BottomBar.vue"
</script>

<style>
</style>
