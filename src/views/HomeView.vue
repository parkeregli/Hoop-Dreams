<template>
  <div class="flex align-items-stretch justify-content-between h-22rem">
    <div class="w-full h-full">
      <TeamDisplay class="h-full" :players="homePlayers" :name="'Home'" />
    </div>
    <div class="w-full h-full">
      <TeamStats class="h-full" :teamStats="teamStats" :name="'Team Stats'" />
    </div>
    <div class="w-full h-full">
      <TeamDisplay class="h-full" :players="awayPlayers" :name="'Away'" />
    </div>
  </div>

  <div class="absolute bottom-0 justify-content-center w-full">
    <GameCast />
  </div>
</template>

<script setup lang="ts">
import GameCast from "@/components/GameCast.vue";
import TeamDisplay from "@/components/TeamDisplay.vue";
import TeamStats from "@/components/TeamStats.vue";
const homePlayers = [
  { position: "PG", name: "Player Name" },
  { position: "SG", name: "Player Name" },
  { position: "SF", name: "Player Name" },
  { position: "PF", name: "Player Name" },
  { position: "C", name: "Player Name" },
];

const awayPlayers = [
  { position: "PG", name: "Player Name" },
  { position: "SG", name: "Player Name" },
  { position: "SF", name: "Player Name" },
  { position: "PF", name: "Player Name" },
  { position: "C", name: "Player Name" },
];

const teamStats = [
  { stat: "Team", home: "Home Logo", name: "Away Logo" },
  { stat: "FG", home: "Home", name: "Away" },
  { stat: "FG %", home: "Home", name: "Away" },
  { stat: "3PT", home: "Home", name: "Away" },
  { stat: "3PT %", home: "Home", name: "Away" },
  { stat: "FT", home: "Home", name: "Away" },
  { stat: "FT  %", home: "Home", name: "Away" },
  { stat: "REB", home: "Home", name: "Away" },
  { stat: "TO", home: "Home", name: "Away" },
];
</script>

<style scoped>
.row {
  width: 100%;
  height: 100%;
}
</style>
