<template>
  <card class="homeAway border-solid border-cyan-600">
    <template #title>
      <h1 class="title">{{ team.name }}</h1>
    </template>
    <template #content>
      <DataView :value="players">
        <template #list="slotProps">
          <div class="flex flex-column">
            <div v-for="(item, index) in slotProps.items" :key="index">
              <span class="position"> {{ item.position }} </span> -
              {{ item.first_name }} {{ item.last_name }}
            </div>
          </div>
        </template>
      </DataView>
    </template>
  </card>
</template>

<script setup lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { ref, onMounted, watch } from "vue";
  const props = defineProps({ teamId: { type: Number, required: true, default: null } });

  const team = ref({ name: "Team Name" });
  const players = ref([
    { position: "PG", first_name: "Player", last_name: "Name" },
    { position: "SG", first_name: "Player", last_name: "Name" },
    { position: "SF", first_name: "Player", last_name: "Name" },
    { position: "PF", first_name: "Player", last_name: "Name" },
    { position: "C", first_name: "Player", last_name: "Name" },
  ])

  const getTeams = async () => {
    try {
      const teamRes = await invoke("get_team", { teamId: props.teamId });
      team.value = teamRes;
    } catch (error) {
      console.error(error);
    }
  }

  const getStartingLineup = async () => {
    try {
      const startingLineupRes = await invoke("get_team_starting_lineup", { teamId: props.teamId });
      players.value = startingLineupRes;
    } catch (error) {
      console.error(error);
    }
  }

  watch(() => props.teamId, async () => {
    if (props.teamId) {
      await getTeams();
      await getStartingLineup();
    }
  });

  onMounted(async () => {
    if (props.teamId) {
      await getTeams();
      await getStartingLineup();
    }
  });
</script>

<style scoped>
.title {
  text-align: center;
}
</style>
