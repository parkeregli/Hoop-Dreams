<template>
  <div class="row flex align-items-center just-content-center">
    <div class="w-full">
      <TeamDisplay :players="homePlayers" :name="'Home'" />
    </div>
    <div class="w-full">
      <TeamStats :players="awayPlayers" :name="'Team Stats'" />
    </div>
    <div class="w-full">
      <TeamDisplay :players="awayPlayers" :name="'Away'" />
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
  { position: "PG", name: "Player Name" },
  { position: "SG", name: "Player Name" },
  { position: "SF", name: "Player Name" },
  { position: "PF", name: "Player Name" },
  { position: "C", name: "Player Name" },
];
</script>

<style scoped>
.row {
  width: 100%;
  height: 100%;
}
</style>
