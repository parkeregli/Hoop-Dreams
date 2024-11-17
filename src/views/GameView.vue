<template>
  <div class="flex flex-column justify-content-between h-screen">
    <Toolbar></Toolbar>
    <div v-if="teams.length > 0" class="flex flex-grow">
      <div class="w-full">
        <TeamDisplay class="h-full" :score="game.state.score[0]" :team_name="teams[0].name"
          :players="game.state.team_state[0].active_players" />
      </div>
      <div class="flex justify-center p-5 col-6">
        <CourtSim />
      </div>
      <div class="w-full">
        <TeamDisplay class="h-full" :score="game.state.score[1]" :team_name="teams[1].name"
          :players="game.state.team_state[1].active_players" />
      </div>
    </div>
    <GameCast />
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted, computed } from "vue";
import GameCast from "@/components/GameCast.vue";
import TeamDisplay from "@/components/TeamDisplay.vue";
import TeamStats from "@/components/TeamStats.vue";
import { useRouter, useRoute } from "vue-router";
import Toolbar from "@/components/Toolbar.vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import CourtSim from "@/components/CourtSim.vue";
const appWindow = getCurrentWebviewWindow()
const unlisten = appWindow.listen('game_score', (event) => {
  game.value.state.score = event.payload
})

const router = useRouter();

const teams = ref([]);
const game = ref({});
const loading = ref(false);

const loadGame = async () => {
  try {
    loading.value = true;
    const gameRes = await invoke("load_game");
    if (gameRes === null) {
      throw new Error("Failed to load game");
    }
    game.value = gameRes;
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

<style scoped></style>
