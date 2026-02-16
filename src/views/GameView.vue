<template>
  <div class="flex flex-column justify-content-between h-screen">
    <Toolbar></Toolbar>
    <div v-if="hasTeams" class="flex flex-grow">
      <div class="w-full">
        <div class="flex justify-content-center gap-2 mb-2">
          <Button 
            size="small" 
            :variant="homeViewMode === 'roster' ? 'filled' : 'outlined'"
            @click="homeViewMode = 'roster'"
          >Roster</Button>
          <Button 
            size="small" 
            :variant="homeViewMode === 'attributes' ? 'filled' : 'outlined'"
            @click="homeViewMode = 'attributes'"
          >Attributes</Button>
        </div>
        <TeamDisplay
          class="h-full"
          :score="homeScore"
          :team_name="homeTeamName"
          :players="homePlayers"
          :bench="homeBench"
          :viewMode="homeViewMode"
        />
      </div>
      <div class="flex flex-column justify-center align-items-center p-5 col-6">
        <Scoreboard
          :time="gameTime"
          :shot-clock="shotClock"
          :period="period"
          :possession="possession"
          class="mb-4"
        />
        <CourtSim />
      </div>
      <div class="w-full">
        <div class="flex justify-content-center gap-2 mb-2">
          <Button 
            size="small" 
            :variant="awayViewMode === 'roster' ? 'filled' : 'outlined'"
            @click="awayViewMode = 'roster'"
          >Roster</Button>
          <Button 
            size="small" 
            :variant="awayViewMode === 'attributes' ? 'filled' : 'outlined'"
            @click="awayViewMode = 'attributes'"
          >Attributes</Button>
        </div>
        <TeamDisplay
          class="h-full"
          :score="awayScore"
          :team_name="awayTeamName"
          :players="awayPlayers"
          :bench="awayBench"
          :viewMode="awayViewMode"
        />
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
import Toolbar from "@/components/Toolbar.vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import CourtSim from "@/components/CourtSim.vue";
import Scoreboard from "@/components/Scoreboard.vue";
import Button from "primevue/button";
import type { Game, Team, Player, GameScorePayload, GameClockPayload } from "@/types/game";

const appWindow = getCurrentWebviewWindow();

// State
const teams = ref<Team[]>([]);
const game = ref<Game | null>(null);
const loading = ref(false);
const homeViewMode = ref<"roster" | "attributes">("roster");
const awayViewMode = ref<"roster" | "attributes">("roster");

// Computed properties for cleaner template access
const hasTeams = computed(() => teams.value.length >= 2 && game.value !== null);
const homeTeamName = computed(() => teams.value[0]?.name ?? "");
const awayTeamName = computed(() => teams.value[1]?.name ?? "");
const homeScore = computed(() => game.value?.state.score[0] ?? 0);
const awayScore = computed(() => game.value?.state.score[1] ?? 0);
const homePlayers = computed(() => game.value?.state.team_state[0].active_players ?? []);
const awayPlayers = computed(() => game.value?.state.team_state[1].active_players ?? []);
const homeBench = computed((): [Player[], Player[]] => game.value?.state.team_state[0].bench ?? [[], []]);
const awayBench = computed((): [Player[], Player[]] => game.value?.state.team_state[1].bench ?? [[], []]);
const gameTime = computed(() => game.value?.state.time ?? { secs: 0, nanos: 0 });
const shotClock = computed(() => game.value?.state.shot_clock ?? { secs: 0, nanos: 0 });
const period = computed(() => game.value?.state.period ?? 1);
const possession = computed(() => game.value?.state.possession ?? null);

// Listen for score updates
appWindow.listen<GameScorePayload>("game_score", (event) => {
  if (game.value) {
    game.value.state.score = event.payload;
  }
});

// Listen for clock updates
appWindow.listen<GameClockPayload>("game_clock", (event) => {
  if (game.value) {
    game.value.state.time = event.payload[0];
    game.value.state.shot_clock = event.payload[1];
    game.value.state.period = event.payload[2];
    game.value.state.possession = event.payload[3];
  }
});

/**
 * Loads the game from the backend.
 */
async function loadGame(): Promise<void> {
  try {
    loading.value = true;
    homeViewMode.value = "roster";
    awayViewMode.value = "roster";
    const gameRes = await invoke<Game>("load_game");

    if (gameRes === null) {
      throw new Error("Failed to load game");
    }

    game.value = gameRes;
    teams.value = [gameRes.teams[0], gameRes.teams[1]];
  } catch (error) {
    console.error("Failed to load game:", error);
  } finally {
    loading.value = false;
  }
}

onMounted(async () => {
  await loadGame();
});
</script>

<style scoped></style>
