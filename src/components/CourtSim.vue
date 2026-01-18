<template>
  <div class="relative">
    <img :src="CourtTemplate" class="h-full w-full" ref="imageRef" />
    <canvas ref="canvasRef" class="testCanvas absolute top-0 left-0 h-full w-full"></canvas>
  </div>
</template>

<script lang="ts" setup>
import CourtTemplate from "@/assets/images/court_template.png";
import { ref, onMounted } from 'vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import {
  COURT_SECTIONS,
  flipSections,
  percentToPixels,
  findSection,
  type CourtSection,
} from "@/data/court-sections";
import type {
  Player,
  PlayerState,
  PlayerStatesPayload,
  GameEvent,
} from "@/types/game";

interface DisplayPlayer {
  id: number;
  first_name: string;
  last_name: string;
  state: PlayerState;
  teamIndex: number; // 0 = home, 1 = away
}

// Team colors for player rectangles
const TEAM_COLORS = {
  home: "rgba(0, 100, 200, 0.9)",  // Blue for home team
  away: "rgba(200, 50, 50, 0.9)",  // Red for away team
};

const appWebview = getCurrentWebviewWindow();

const imageRef = ref<HTMLImageElement>();
const canvasRef = ref<HTMLCanvasElement>();
const players = ref<DisplayPlayer[]>([]);
const event = ref<GameEvent>();
const flipState = ref(false);

// Listen for player state updates
appWebview.listen<PlayerStatesPayload>('player_states', (eventData) => {
  const canvas = canvasRef.value;
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // Build list of display players
  const displayPlayers: DisplayPlayer[] = [];

  // Backend sends home team first (indices 0-4), then away team (indices 5-9)
  eventData.payload.forEach(([playerData, playerState], index) => {
    const displayPlayer: DisplayPlayer = {
      first_name: playerData.first_name,
      last_name: playerData.last_name,
      id: playerData.id,
      state: playerState,
      teamIndex: index < 5 ? 0 : 1, // First 5 = home, last 5 = away
    };

    updateOrAddPlayer(displayPlayer);
    displayPlayers.push(displayPlayer);
  });

  // Group players by their current area
  const playersByArea = new Map<string, DisplayPlayer[]>();
  for (const player of displayPlayers) {
    const area = player.state.current_area;
    if (!playersByArea.has(area)) {
      playersByArea.set(area, []);
    }
    playersByArea.get(area)!.push(player);
  }

  // Draw each group with proper positioning
  for (const [area, areaPlayers] of playersByArea) {
    drawPlayersInSection(ctx, areaPlayers);
  }
});

// Listen for game events to track possession changes
appWebview.listen<GameEvent>('game_event', (newEvent) => {
  if (!event.value) {
    event.value = newEvent.payload;
    return;
  }

  if (event.value.possession !== newEvent.payload.possession) {
    flipState.value = !flipState.value;
  }

  event.value = newEvent.payload;
});

/**
 * Updates an existing player or adds a new one to the players list.
 */
function updateOrAddPlayer(player: DisplayPlayer): void {
  const existingIndex = players.value.findIndex(p => p.id === player.id);

  if (existingIndex >= 0) {
    players.value[existingIndex] = player;
  } else {
    players.value.push(player);
  }
}

// Player rectangle dimensions
const PLAYER_RECT_WIDTH = 20;
const PLAYER_RECT_HEIGHT = 14;
const PLAYER_SPACING = 4; // Gap between players in same section
const PLAYER_TOTAL_HEIGHT = PLAYER_RECT_HEIGHT + 14; // Rectangle + name space

/**
 * Draws multiple players in the same section, arranged side by side.
 */
function drawPlayersInSection(ctx: CanvasRenderingContext2D, players: DisplayPlayer[]): void {
  if (players.length === 0) return;

  const sections = flipState.value ? flipSections(COURT_SECTIONS) : COURT_SECTIONS;
  const section = findSection(sections, players[0].state.current_area);

  if (!section) return;

  // Get section position and size
  const sectionX = percentToPixels(section.x, ctx.canvas.width);
  const sectionY = percentToPixels(section.y, ctx.canvas.height);
  const sectionW = percentToPixels(section.w, ctx.canvas.width);
  const sectionH = percentToPixels(section.h, ctx.canvas.height);

  // Calculate total width needed for all players
  const totalWidth = players.length * PLAYER_RECT_WIDTH + (players.length - 1) * PLAYER_SPACING;

  // Starting X position to center the group
  const startX = sectionX + (sectionW - totalWidth) / 2;
  const centerY = sectionY + (sectionH - PLAYER_TOTAL_HEIGHT) / 2;

  // Draw each player
  players.forEach((player, index) => {
    const rectX = startX + index * (PLAYER_RECT_WIDTH + PLAYER_SPACING);
    const rectY = centerY;

    drawPlayerRect(ctx, player, rectX, rectY);
  });
}

/**
 * Draws a single player rectangle with their last name at the specified position.
 */
function drawPlayerRect(ctx: CanvasRenderingContext2D, player: DisplayPlayer, rectX: number, rectY: number): void {
  // Draw player rectangle with team color
  const teamColor = player.teamIndex === 0 ? TEAM_COLORS.home : TEAM_COLORS.away;
  ctx.fillStyle = teamColor;
  ctx.fillRect(rectX, rectY, PLAYER_RECT_WIDTH, PLAYER_RECT_HEIGHT);

  // Draw border around rectangle
  ctx.strokeStyle = "white";
  ctx.lineWidth = 1;
  ctx.strokeRect(rectX, rectY, PLAYER_RECT_WIDTH, PLAYER_RECT_HEIGHT);

  // Draw player last name below the rectangle
  ctx.fillStyle = "white";
  ctx.font = "bold 10px sans-serif";
  ctx.textAlign = "center";
  ctx.textBaseline = "top";

  // Add text shadow for better visibility
  ctx.shadowColor = "black";
  ctx.shadowBlur = 2;
  ctx.shadowOffsetX = 1;
  ctx.shadowOffsetY = 1;

  ctx.fillText(player.last_name, rectX + PLAYER_RECT_WIDTH / 2, rectY + PLAYER_RECT_HEIGHT + 2);

  // Reset shadow
  ctx.shadowColor = "transparent";
  ctx.shadowBlur = 0;
  ctx.shadowOffsetX = 0;
  ctx.shadowOffsetY = 0;
}

/**
 * Draws all court sections for debugging/visualization.
 */
function drawCourtSections(ctx: CanvasRenderingContext2D): void {
  for (const section of COURT_SECTIONS) {
    const x = percentToPixels(section.x, ctx.canvas.width);
    const y = percentToPixels(section.y, ctx.canvas.height);
    const w = percentToPixels(section.w, ctx.canvas.width);
    const h = percentToPixels(section.h, ctx.canvas.height);
    ctx.fillStyle = section.color;
    ctx.fillRect(x, y, w, h);
  }
}

onMounted(() => {
  const canvas = canvasRef.value;
  const img = imageRef.value;

  if (canvas && img) {
    img.onload = () => {
      canvas.width = img.width;
      canvas.height = img.height;
    };
  }
});
</script>

<style scoped>
canvas {
  pointer-events: none;
}
</style>
