<template>
  <div class="row flex flex-column align-items-center justify-content-center">
    <div class="absolute bottom-0 justify-content-center w-full">
      <GameCast />
    </div>
  </div>
</template>

<script setup lang="ts">
import GameCast from "@/components/GameCast.vue";
</script>

<style scoped>
.row {
  width: 100%;
  height: 100%;
}
</style>
