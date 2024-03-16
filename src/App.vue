<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import {appWindow} from '@tauri-apps/api/window'
document
  .getElementById('titlebar-minimize')
  .addEventListener('click', () => appWindow.minimize())
document
  .getElementById('titlebar-maximize')
  .addEventListener('click', () => appWindow.toggleMaximize())
document
  .getElementById('titlebar-close')
  .addEventListener('click', () => appWindow.close())
</script>

<template>
  <!-- Don't delete this -->
  <RouterView />
</template>
