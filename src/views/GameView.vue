<template>
  <div class="flex-column justify-content-between align-content-between">
    <div class="w-full">
      <Toolbar></Toolbar>
      <div v-if="teams.length > 0" class="row align-items-stretch justify-content-between h-22rem">
        <div class="w-full h-full">
          <TeamDisplay class="h-full" :score="game.state.score[0]" :team_name="teams[0].name"
            :players="game.state.team_state[0].active_players" />
        </div>
        <div v-if="false" class="w-full h-full">
          <TeamStats class="h-full" :teamStats="teamStats" />
        </div>
        <div class="w-full h-full">
          <TeamDisplay class="h-full" :score="game.state.score[1]" :team_name="teams[1].name"
            :players="game.state.team_state[1].active_players" />
        </div>
      </div>
    </div>
    <div class="court-wrapper flex justify-center m-auto my-4">
      <CourtSim />
    </div>
    <div class="game-cast-wrapper" style="height: 350px">
      <GameCast />
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";
import GameCast from "@/components/GameCast.vue";
import TeamDisplay from "@/components/TeamDisplay.vue";
import TeamStats from "@/components/TeamStats.vue";
import { useRouter, useRoute } from "vue-router";
import Toolbar from "@/components/Toolbar.vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import CourtSim from "@/components/CourtSim.vue";
const appWindow = getCurrentWebviewWindow()
const unlisten = appWindow.listen('game_score', (event) => {
  console.log(event.payload)
  game.value.state.score = event.payload
})

const router = useRouter();

const teams = ref([]);
const game = ref({});
const loading = ref(false);

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
const loadGame = async () => {
  try {
    loading.value = true;
    const gameRes = await invoke("load_game");
    if (gameRes === null) {
      throw new Error("Failed to load game");
    }
    game.value = gameRes;
    console.log(game.value.state.score);
    teams.value = gameRes.teams;
  } catch (error) {
    console.error(error);
  }
};

onMounted(async () => {
  try {
    await loadGame();
  } catch (err) {
    console.error(err);
  }
});
</script>

<style scoped>
.game-cast-wrapper {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.court-wrapper {
  max-height: 50%;
  max-width: 50%;
}
</style>
