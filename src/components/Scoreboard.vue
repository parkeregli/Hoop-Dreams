<template>
  <div class="scoreboard">
    <div class="scoreboard-section possession-section">
      <span class="arrow" :class="{ active: isHomePossession }">&larr;</span>
    </div>
    <div class="scoreboard-section period-section">
      <span class="label">PERIOD</span>
      <span class="value period-value">{{ periodDisplay }}</span>
    </div>
    <div class="scoreboard-section time-section">
      <span class="label">TIME</span>
      <span class="value time-value">{{ gameTimeDisplay }}</span>
    </div>
    <div class="scoreboard-section shot-clock-section">
      <span class="label">SHOT</span>
      <span class="value shot-clock-value">{{ shotClockDisplay }}</span>
    </div>
    <div class="scoreboard-section possession-section">
      <span class="arrow" :class="{ active: isAwayPossession }">&rarr;</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { Possession } from "@/types/game";

const props = defineProps<{
  time: { secs: number; nanos: number };
  shotClock: { secs: number; nanos: number };
  period: number;
  possession: [Possession, number] | null;
}>();

const isHomePossession = computed(() => props.possession?.[0] === "Home");
const isAwayPossession = computed(() => props.possession?.[0] === "Away");

const periodDisplay = computed(() => {
  const p = props.period;
  if (p === 1) return "1ST";
  if (p === 2) return "2ND";
  if (p === 3) return "3RD";
  if (p === 4) return "4TH";
  if (p > 4) return `OT${p - 4}`;
  return String(p);
});

const gameTimeDisplay = computed(() => {
  const totalSeconds = props.time.secs;
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  const tenths = Math.floor(props.time.nanos / 100000000);

  // Show tenths when under 1 minute
  if (minutes === 0) {
    return `${seconds}.${tenths}`;
  }
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
});

const shotClockDisplay = computed(() => {
  const totalSeconds = props.shotClock.secs;
  const tenths = Math.floor(props.shotClock.nanos / 100000000);

  // Show tenths when under 10 seconds
  if (totalSeconds < 10) {
    return `${totalSeconds}.${tenths}`;
  }
  return String(totalSeconds);
});
</script>

<style scoped>
.scoreboard {
  display: flex;
  justify-content: center;
  align-items: stretch;
  gap: 2px;
  background: #1a1a1a;
  border: 2px solid #333;
  border-radius: 4px;
  padding: 4px;
  font-family: "Courier New", monospace;
}

.scoreboard-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  background: #0a0a0a;
  border: 1px solid #222;
}

.label {
  font-size: 0.65rem;
  color: #888;
  letter-spacing: 0.1em;
  margin-bottom: 2px;
}

.value {
  font-weight: bold;
  color: #ff4444;
  text-shadow: 0 0 8px rgba(255, 68, 68, 0.5);
}

.period-section {
  min-width: 60px;
}

.period-value {
  font-size: 1.2rem;
}

.time-section {
  min-width: 100px;
}

.time-value {
  font-size: 1.8rem;
}

.shot-clock-section {
  min-width: 50px;
}

.shot-clock-value {
  font-size: 1.2rem;
  color: #ffaa00;
  text-shadow: 0 0 8px rgba(255, 170, 0, 0.5);
}

.possession-section {
  min-width: 40px;
  padding: 8px 12px;
}

.arrow {
  font-size: 1.5rem;
  color: #333;
  transition: color 0.2s, text-shadow 0.2s;
}

.arrow.active {
  color: #00ff00;
  text-shadow: 0 0 10px rgba(0, 255, 0, 0.7);
}
</style>
