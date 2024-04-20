<template>
  <Toolbar><template #end>
      <div class="flex align-items-center gap-3 h-1rem">
        <SplitButton label="Options" menuButtonIcon="pi pi-cog" :model="items" />
        <router-link to="/">
          <Button label="Submit" class="p-button-icon"><i class="pi pi-times"></i></Button></router-link>
      </div>
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
const items = [
  { label: "Settings", icon: "pi-bar" },
  { label: "Sound", icon: "pi-bar" },
  { label: "Help Me", icon: "pi-bar" },
  { label: "Report a Bug", icon: "pi-bar" },
];
</script>

<style scoped></style>
