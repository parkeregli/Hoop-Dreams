<template>
  <Card class="h-full">
    <template #title>
      <div class="flex justify-content-center">
        <h3 class="title">Game Cast</h3>
      </div>
      <div class="flex flex-wrap gap-3">
        <div class="flex justify-content-center text-white">
          <SelectButton @click="handleSimSpeed()" v-model="selected" :options="options" />
        </div>
      </div>
      <div class="flex flex-wrap gap-3 mt-3">
        <div class="flex justify-content-center text-white">
          <SelectButton 
            @click="handleJumpTarget()" 
            v-model="jumpSelected" 
            :options="jumpOptions" 
            :disabled="isSimming"
          />
        </div>
      </div>
    </template>
    <template #content>
      <div class="flex flex-column flex-1" style="height: 30vh">
        <DataTable size="small" :value="items" class="flex flex-column p-0 flex-1" scrollable scrollHeight="flex"
          tableStyle="min-width: 50rem">
          <Column field="possession" header="Poss"></Column>
          <Column field="period" header="Per"></Column>
          <Column field="time" header="Time"></Column>
          <Column field="action" header="Action"></Column>
        </DataTable>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { JumpTarget } from '@/types/game';

const items = ref([]);
const appWebview = getCurrentWebviewWindow();
const isSimming = ref(false);
const checked = ref(false);
const selected = ref(null);
const jumpSelected = ref(null);

const options = [
  'Stop',
  'Play',
  'x2',
  'x3',
]

const jumpOptions = [
  'Q2 Start',
  'Q3 Start',
  'Q4 Start',
  'End Game',
]

const unlisten = appWebview.listen('game_event', (event) => {
  items.value.unshift(event.payload);
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

const handleJumpTarget = async () => {
  const targetMap: Record<string, JumpTarget> = {
    'Q2 Start': 'Q2Start',
    'Q3 Start': 'Q3Start',
    'Q4 Start': 'Q4Start',
    'End Game': 'GameEnd',
  };
  
  if (jumpSelected.value && targetMap[jumpSelected.value]) {
    await invoke("jump_to_target", { target: targetMap[jumpSelected.value] });
    jumpSelected.value = null;
  }
}
</script>

<style scoped>
:deep(.p-datatable-wrapper) {
  height: 100%;
}

:deep(.p-datatable-table) {
  height: 100%;
}
</style>
