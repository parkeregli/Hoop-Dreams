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
const appWebview = getCurrentWebviewWindow();
const unlistenPlayerStates = appWebview.listen('player_states', (event) => {
  //Update the canvas
  const canvas = canvasRef.value;
  if (canvas) {
    const ctx = canvas.getContext('2d');
    if (ctx) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      for (let i = 0; i < event.payload.length; i++) {
        const player = {
          first_name: event.payload[i][0].first_name,
          last_name: event.payload[i][0].last_name,
          id: event.payload[i][0].id,
          state: event.payload[i][1]
        }

        if (players.value.length == 0) {
          players.value.push(player);
        } else {
          //Replace the player with the new state if it already exists
          for (let j = 0; j < players.value.length; j++) {
            if (players.value[j].id == player.id) {
              players.value[j] = player;
              break;
            }
            if (j == players.value.length - 1) {
              players.value.push(player);
            }
          }
        }
        console.log(player);
        drawPlayer(ctx, player);
      }
    }
  }
});

//Store the last two events and see if the possession changed
const event = ref();
const flipState = ref(false);
const unlistenEvents = appWebview.listen('game_event', (newEvent) => {
  if (!event.value) {
    event.value = newEvent.payload;
    return;
  }
  if (event.value.possession !== newEvent.payload.possession) {
    flipState.value = !flipState.value;
  }
  event.value = newEvent.payload;
})

const imageRef = ref<HTMLImageElement>();
const canvasRef = ref<HTMLCanvasElement>();

const players = ref([]);

interface Section {
  name: string;
  x: number;
  y: number;
  w: number;
  h: number;
  color: string;
}

interface Player {
  id: number;
  first_name: string;
  last_name: string;
  state: PlayerState;
}

interface PlayerState {
  action: string;
  current_area: string;
}

const sections: Section[] = [
  { name: "ThreePointLineCornerRight", x: 0, y: 0, w: 15, h: 12, color: "rgba(255, 0, 0, 0.5)" },
  { name: "ThreePointLineWingRight", x: 15, y: 0, w: 16, h: 24, color: "rgba(255, 255, 0, 0.5)" },
  { name: "MidrangeBaselineRight", x: 0, y: 12, w: 7, h: 16, color: "rgba(0, 255, 0, 0.5)" },
  { name: "MidrangeWingRight", x: 7, y: 12, w: 8, h: 16, color: "rgba(0, 0, 255, 0.5)" },
  { name: "ShortCornerRight", x: 0, y: 28, w: 6, h: 11, color: "rgba(0, 0, 255, 0.5)" },
  { name: "LowPostRight", x: 6, y: 28, w: 9, h: 11, color: "rgba(255, 255, 0, 0.5)" },
  { name: "RestrictedAreaRight", x: 0, y: 39, w: 8, h: 6, color: "rgba(255, 0, 0, 0.5)" },
  { name: "ElbowRight", x: 15, y: 28, w: 5, h: 11, color: "rgba(255, 0, 0, 0.5)" },
  { name: "FreeThrowLine", x: 12, y: 39, w: 8, h: 22, color: "rgba(0, 255, 0, 0.5)" },
  { name: "RestrictedAreaMiddle", x: 7, y: 45, w: 4, h: 11, color: "rgba(0, 255, 0, 0.5)" },
  { name: "MidrangeCenter", x: 20, y: 24, w: 7, h: 52, color: "rgba(0, 0, 255, 0.5)" },
  { name: "ThreePointLineCenter", x: 27, y: 24, w: 11, h: 52, color: "rgba(0, 255, 0, 0.5)" },
  { name: "Center", x: 38, y: 0, w: 25, h: 100, color: "rgba(255, 0, 0, 0.5)" },
  { name: "Backcourt", x: 63, y: 0, w: 37, h: 100, color: "rgba(255, 255, 0, 0.5)" },
  { name: "ThreePointLineWingLeft", x: 0, y: 88, w: 15, h: 12, color: "rgba(255, 255, 0, 0.5)" },
  { name: "ThreePointLineWingLeft", x: 15, y: 76, w: 16, h: 24, color: "rgba(255, 0, 0, 0.5)" },
  { name: "MidrangeBaselineLeft", x: 0, y: 72, w: 7, h: 16, color: "rgba(0, 255, 0, 0.5)" },
  { name: "MidrangeWingLeft", x: 7, y: 74, w: 8, h: 14, color: "rgba(0, 0, 255, 0.5)" },
  { name: "ShortCornerLeft", x: 0, y: 61, w: 6, h: 11, color: "rgba(0, 0, 255, 0.5)" },
  { name: "LowPostLeft", x: 6, y: 61, w: 9, h: 11, color: "rgba(255, 255, 0, 0.5)" },
  { name: "RestrictedAreaLeft", x: 0, y: 56, w: 8, h: 5, color: "rgba(0, 255, 255, 0.5)" },
  { name: "ElbowLeft", x: 15, y: 61, w: 5, h: 11, color: "rgba(255, 0, 0, 0.5)" },
]

function flipSections(sections: Section[]) {
  return sections.map(section => {
    return {
      name: section.name,
      x: 100 - section.x - section.w,
      y: section.y,
      w: section.w,
      h: section.h,
      color: section.color
    }
  })
}

function percentToPixels(percent: number, total: number) {
  return (percent / 100) * total;
}

//Get a player and draw a rectangle with their initials inside the section
function drawPlayer(ctx: CanvasRenderingContext2D, player: Player) {
  let section;
  if (flipState.value) {
    section = flipSections(sections).find(section => section.name == player.state.current_area);
  } else {
    section = sections.find(section => section.name == player.state.current_area);
  }
  if (!section) {
    return;
  }
  const x = percentToPixels(section.x, ctx.canvas.width);
  const y = percentToPixels(section.y, ctx.canvas.height);
  const w = percentToPixels(section.w, ctx.canvas.width);
  const h = percentToPixels(section.h, ctx.canvas.height);
  ctx.fillStyle = section.color;
  ctx.fillRect(x, y, w, h);
  ctx.fillStyle = "black";
  ctx.fillText(player.first_name[0], x + w / 2, y + h / 2);
  ctx.fillText(player.last_name[0], x + w / 2, y + h / 2 + 10);
}

onMounted(() => {
  const canvas = canvasRef.value;
  const img = imageRef.value;

  if (canvas && img) {
    // Wait for image to load to get correct dimensions
    img.onload = () => {
      // Set canvas dimensions to match image
      canvas.width = img.width;
      canvas.height = img.height;
      // Now you can draw on the canvas
      const ctx = canvas.getContext('2d');
      if (ctx) {
        //Set fill to red with 50% transparency
        for (const section of sections) {
          const x = percentToPixels(section.x, canvas.width);
          const y = percentToPixels(section.y, canvas.height);
          const w = percentToPixels(section.w, canvas.width);
          const h = percentToPixels(section.h, canvas.height);
          ctx.fillStyle = section.color;
          ctx.fillRect(x, y, w, h);
        }
      }
    };
  }
});
</script>

<style scoped>
canvas {
  pointer-events: none;
}
</style>
