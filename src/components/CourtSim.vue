<template>
  <div class="relative">
    <img :src="CourtTemplate" class="h-full w-full" ref="imageRef" />
    <canvas ref="canvasRef" class="testCanvas absolute top-0 left-0 h-full w-full"></canvas>
  </div>
</template>

<script lang="ts" setup>
import CourtTemplate from "@/assets/images/court_template.png";
import { ref, onMounted } from 'vue';

const imageRef = ref<HTMLImageElement>();
const canvasRef = ref<HTMLCanvasElement>();

interface Section {
  name: string;
  x: number;
  y: number;
  w: number;
  h: number;
  color: string;
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
