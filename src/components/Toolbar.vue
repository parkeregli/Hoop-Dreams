<template>
  <Toolbar
    ><template #end>
      <div class="flex align-items-center gap-3 h-1rem">
        <SplitButton
          label="Options"
          menuButtonIcon="pi pi-cog"
          @click="save"
          :model="items"
        />
        <Button label="Submit" class="p-button-icon"
          ><i class="pi pi-times"></i
        ></Button>
      </div>
    </template>
  </Toolbar>
</template>

<script setup lang="ts">
const items = [{ label: "settings", icon: "pi-bar" }];
</script>

<style scoped></style>
