<template>
  <Card>
    <template #title>
      <h3 class="title">{{ team_name }}: {{ score }}</h3>
    </template>
    <template #content>
      <DataTable size="small" :value="mappedPlayers">
        <Column field="first_name" header="First Name"></Column>
        <Column field="last_name" header="Last Name"></Column>
        <Column field="position" header="Position"></Column>
      </DataTable>
    </template>
  </card>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted, computed } from "vue";
const props = defineProps({
  team_name: { type: String, required: true },
  players: { type: Array, required: true },
  score: { type: Number, required: true }
});
const mappedPlayers = computed(() => {
  return props.players.map((player) => {
    return {
      position: player[0].position,
      first_name: player[0].first_name,
      last_name: player[0].last_name
    };
  });
});
</script>

<style scoped>
.title {
  text-align: center;
}
</style>
