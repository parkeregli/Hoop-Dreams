<template>
  <Card class="card">
    <template #title>
      <div class="flex flex-wrap gap-3">
        <div class="flex justify-content-center text-white">
          <ToggleButton @click="nextEvent()" v-model="checked" class="w-6rem" onLabel="Pause" offLabel="Play" />
        </div>
      </div>

      <h1 class="title">Game Cast</h1>
    </template>
    <template #content>
      <DataView :value="items">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div v-for="(item, index) in slotProps.items" :key="index">
              <span class="timestamp"> {{ item.timestamp }} </span> -
              {{ item.name }}
            </div>
          </div>
        </template>
      </DataView>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/core';
const items = [
  { timestamp: "1:00", name: "Item 1" },
  { timestamp: "2:00", name: "Item 2" },
  { timestamp: "3:00", name: "Item 3" },
];

const nextEvent = () => {
  invoke('next_play');
}

const checked = ref(false);
</script>

<style scoped>
.timestamp {
  border: 2px solid rgba(255, 230, 0, 0.767);
  padding: 2px 5px;
}

.title {
  text-align: center;
}

.card {
  width: 100%;
  height: 200px;
}
</style>
