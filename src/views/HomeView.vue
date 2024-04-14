<template>
  <div class="flex align-items-stretch justify-content-between h-22rem">
    <div class="w-full h-full">
      <TeamDisplay class="h-full" :teamId="teams[0].id" />
    </div>
    <div v-if="false" class="w-full h-full">
      <TeamStats class="h-full" :teamStats="teamStats" />
    </div>
    <div class="w-full h-full">
      <TeamDisplay class="h-full" :teamId="teams[1].id" />
    </div>
  </div>

  <div class="absolute bottom-0 justify-content-center w-full">
    <GameCast />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";
import GameCast from "@/components/GameCast.vue";
import TeamDisplay from "@/components/TeamDisplay.vue";
import TeamStats from "@/components/TeamStats.vue";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();

const teams = [{ name: "Team 1" }, { name: "Team 2" }];
const loading = ref(false);

const awayPlayers = [
  { position: "PG", name: "Player Name" },
  { position: "SG", name: "Player Name" },
  { position: "SF", name: "Player Name" },
  { position: "PF", name: "Player Name" },
  { position: "C", name: "Player Name" },
];

const teamStats = [
  { stat: "Team", home: "Home Logo", away: "Away Logo" },
  { stat: "FG", home: "Home", away: "Away" },
  { stat: "FG %", home: "Home", away: "Away" },
  { stat: "3PT", home: "Home", away: "Away" },
  { stat: "3PT %", home: "Home", away: "Away" },
  { stat: "FT", home: "Home", away: "Away" },
  { stat: "FT  %", home: "Home", away: "Away" },
  { stat: "REB", home: "Home", away: "Away" },
  { stat: "TO", home: "Home", away: "Away" },
];
const getTeams = async () => {
  try {
    loading.value = true;
    const teamRes = await invoke("get_teams");
    if (teamRes === null) {
      throw new Error("No teams found");
    }
    teams = teamRes;
  } catch (error) {
    console.error(error);
  }
};

onMounted(async () => {
  try {
    await getTeams();
  } catch (err) {
    console.error(err);
  }
});
</script>

<style scoped>
.row {
  width: 100%;
  height: 100%;
}
</style>
