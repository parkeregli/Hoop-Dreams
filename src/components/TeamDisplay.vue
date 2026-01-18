<template>
  <Card>
    <template #title>
      <h3 class="title">{{ team_name }}: {{ score }}</h3>
    </template>
    <template #content>
      <h4 class="section-header">Starters</h4>
      <DataTable size="small" :value="mappedPlayers">
        <Column field="first_name" header="First Name"></Column>
        <Column field="last_name" header="Last Name"></Column>
        <Column field="position" header="Position"></Column>
      </DataTable>
      <h4 class="section-header mt-3">Bench</h4>
      <DataTable size="small" :value="mappedBench">
        <Column field="first_name" header="First Name"></Column>
        <Column field="last_name" header="Last Name"></Column>
        <Column field="position" header="Position"></Column>
      </DataTable>
    </template>
  </card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { Player, PlayerState } from "@/types/game";

const props = defineProps<{
  team_name: string;
  players: [Player, PlayerState][];
  bench: [Player[], Player[]];
  score: number;
}>();

const mappedPlayers = computed(() => {
  return props.players.map((player) => {
    return {
      position: player[0].position,
      first_name: player[0].first_name,
      last_name: player[0].last_name
    };
  });
});

const mappedBench = computed(() => {
  // bench is [Player[], Player[]] - flatten both arrays
  const allBench: Player[] = [...(props.bench[0] || []), ...(props.bench[1] || [])];
  return allBench.map((player) => {
    return {
      position: player.position,
      first_name: player.first_name,
      last_name: player.last_name
    };
  });
});
</script>

<style scoped>
.title {
  text-align: center;
}
.section-header {
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  font-weight: 600;
}
</style>
