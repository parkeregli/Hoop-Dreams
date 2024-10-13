<template>
  <Card class="dataview-container">
    <template #title>
      <div class="flex justify-content-center">
        <h1 class="title">Game Cast</h1>
      </div>
      <div class="flex flex-wrap gap-3">
        <div class="flex justify-content-center text-white">
          <ToggleButton @click="toggle()" v-model="checked" class="w-6rem h-3rem" onLabel="Pause" offLabel="Play" />
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

const unlisten = appWebview.listen('game_event', (event) => {
  items.value.push(event.payload);
  if (items.value.length > 100) {
    items.value.shift();
  }
  nextTick(() => {
    scrollToBottom();
  })
});


const toggle = () => {
  if (isSimming.value) {
    invoke("stop_sim");
  } else {
    invoke("start_sim");
  }
  isSimming.value = !isSimming.value;
};

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
