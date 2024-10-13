<template>
  <Card class="dataview-container">
    <template #title>
      <div class="flex justify-content-center">
        <h1 class="title">Game Cast</h1>
      </div>
      <div class="flex flex-wrap gap-3">
        <div class="flex justify-content-center text-white">
          <SelectButton @click="handleSimSpeed()" v-model="selected" :options="options" />
        </div>
      </div>
    </template>
    <template #content>
      <div class="game-cast-content">
        <DataView :value="items" class="p-0 h-full">
          <template #list="slotProps">
            <div class="game-cast-items" ref="scrollContainer">
              <div v-for="(item, index) in slotProps.items" class="game-cast-item" :key="index">
                <span class="timestamp"> {{ item.time }} </span> -
                {{ item.action }}
              </div>
            </div>
          </template>
        </DataView>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

const items = ref([]);
const appWebview = getCurrentWebviewWindow();
const scrollContainer = ref(null);
const isSimming = ref(false);
const checked = ref(false);
const selected = ref(null);

const options = [
  'Stop',
  'Play',
  'x2',
  'x3',
]

const unlisten = appWebview.listen('game_event', (event) => {
  items.value.push(event.payload);
  if (items.value.length > 100) {
    items.value.shift();
  }
  nextTick(() => {
    scrollToBottom();
  })
});

const stopSim = () => {
  if (isSimming.value) {
    invoke("stop_sim");
    isSimming.value = false;
  }
}

const startSim = (speed: number) => {
  if (!isSimming.value) {
    invoke("start_sim", { speed });
    isSimming.value = true;
  } else {
    invoke("set_sim_speed", { speed });
  }
}

const handleSimSpeed = () => {
  switch (selected.value) {
    case "Stop":
      stopSim();
      break;
    case "Play":
      startSim(1);
      break;
    case "x2":
      startSim(2);
      break;
    case "x3":
      startSim(3);
      break;
  }
}

const scrollToBottom = () => {
  if (scrollContainer.value) {
    scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight;
  }
};

onMounted(() => {
  scrollToBottom();
})
</script>

<style scoped>
.timestamp {
  border: 2px solid rgba(255, 230, 0, 0.767);
  padding: 2px 5px;
}

.title {
  text-align: center;
}

.dataview-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.game-cast-content {
  flex: 0 0 auto;
  overflow: hidden;
  min-height: 0;
}

.game-cast-item {
  margin-bottom: 0.5rem;
}

.game-cast-items {
  height: 300px;
  overflow-y: scroll !important;
  padding: 0.5rem;
}
</style>
