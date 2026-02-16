<template>
  <Card>
    <template #title>
      <h3 class="title">{{ team_name }}: {{ score }}</h3>
    </template>
    <template #content>
      <div v-if="viewMode === 'roster'">
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
      </div>
      <div v-else-if="viewMode === 'attributes'">
        <h4 class="section-header">Starters</h4>
        <DataTable size="small" :value="mappedPlayersAttributes" :rows="5">
          <Column field="first_name" header="First"></Column>
          <Column field="last_name" header="Last"></Column>
          <Column field="overall" header="OVR"></Column>
          <Column field="shooting" header="SHT"></Column>
          <Column field="defense" header="DEF"></Column>
          <Column field="physical" header="PHY"></Column>
          <Column field="skills" header="SKL"></Column>
        </DataTable>
        <h4 class="section-header mt-3">Bench</h4>
        <DataTable size="small" :value="mappedBenchAttributes" :rows="5">
          <Column field="first_name" header="First"></Column>
          <Column field="last_name" header="Last"></Column>
          <Column field="overall" header="OVR"></Column>
          <Column field="shooting" header="SHT"></Column>
          <Column field="defense" header="DEF"></Column>
          <Column field="physical" header="PHY"></Column>
          <Column field="skills" header="SKL"></Column>
        </DataTable>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { Player, PlayerState } from "@/types/game";

const props = defineProps<{
  team_name: string;
  players: [Player, PlayerState][];
  bench: [Player[], Player[]];
  score: number;
  viewMode: "roster" | "attributes";
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
  const allBench: Player[] = [...(props.bench[0] || []), ...(props.bench[1] || [])];
  return allBench.map((player) => {
    return {
      position: player.position,
      first_name: player.first_name,
      last_name: player.last_name
    };
  });
});

function calcOverall(attr: Player["attributes"]): number {
  const shooting = (attr.close_shot + attr.mid_shot + attr.deep_shot + attr.shot_in_traffic) / 4;
  const defense = (attr.def_rebound + attr.off_rebound + attr.block + attr.steal + attr.interior_def + attr.perimeter_def) / 6;
  const physical = (attr.spd + attr.ath + attr.strength) / 3;
  const skills = (attr.pass + attr.handle + attr.intelligence) / 3;
  return Math.round((shooting * 0.35 + defense * 0.25 + physical * 0.2 + skills * 0.2));
}

function calcShooting(attr: Player["attributes"]): number {
  return Math.round((attr.close_shot + attr.mid_shot + attr.deep_shot + attr.shot_in_traffic) / 4);
}

function calcDefense(attr: Player["attributes"]): number {
  return Math.round((attr.def_rebound + attr.off_rebound + attr.block + attr.steal + attr.interior_def + attr.perimeter_def) / 6);
}

function calcPhysical(attr: Player["attributes"]): number {
  return Math.round((attr.spd + attr.ath + attr.strength) / 3);
}

function calcSkills(attr: Player["attributes"]): number {
  return Math.round((attr.pass + attr.handle + attr.intelligence) / 3);
}

const mappedPlayersAttributes = computed(() => {
  return props.players.map((player) => {
    const attr = player[0].attributes;
    return {
      first_name: player[0].first_name,
      last_name: player[0].last_name,
      overall: calcOverall(attr),
      shooting: calcShooting(attr),
      defense: calcDefense(attr),
      physical: calcPhysical(attr),
      skills: calcSkills(attr),
    };
  });
});

const mappedBenchAttributes = computed(() => {
  const allBench: Player[] = [...(props.bench[0] || []), ...(props.bench[1] || [])];
  return allBench.map((player) => {
    const attr = player.attributes;
    return {
      first_name: player.first_name,
      last_name: player.last_name,
      overall: calcOverall(attr),
      shooting: calcShooting(attr),
      defense: calcDefense(attr),
      physical: calcPhysical(attr),
      skills: calcSkills(attr),
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
