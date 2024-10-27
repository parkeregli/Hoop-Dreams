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
        // Example: Draw a rectangle
        ctx.fillStyle = 'rgba(255, 0, 0, 0.5)';
        ctx.fillRect(0, 0, 25, 25);
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
